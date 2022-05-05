use crate::ast::Expr;
use crate::ast::Expr::BinOp;
use crate::ast::Expr::Block;
use crate::ast::Expr::Identifier;
use crate::ast::Expr::IfElse;
use crate::ast::Expr::Let;
use crate::ast::Expr::Literal;
use crate::ast::Expr::Match;
use crate::ast::Literal::Float;
use crate::ast::Literal::Integer;
use crate::interner::intern;
use crate::lexer::Lexer;
use crate::lexer::Token::TFloat;
use crate::lexer::Token::TIdentifier;
use crate::lexer::Token::TInteger;
use crate::lexer::Token::TOperator;
use crate::lexer::Token::TString;
use ast::*;
use interner::InternedStr;
use lexer::*;

macro_rules! expect {
    ($e: expr, $p: pat) => {
        match $e.lexer.next() {
            x @ &$p => x,
            x => return Err(format!("Unexpected token {}", x)),
        }
    };
}

macro_rules! matches {
    ($e: expr, $p: pat) => {
        match $e {
            $p => true,
            _ => false,
        }
    };
}

fn precedence(s: &str) -> i32 {
    match s {
        "+" => 1,
        "-" => 1,
        "*" => 3,
        "/" => 3,
        "%" => 3,
        "==" => 1,
        "/=" => 1,
        "<" => 1,
        ">" => 1,
        "<=" => 1,
        ">=" => 1,
        _ => 9,
    }
}

fn is_statement<T>(e: &Expr<T>) -> bool {
    match *e {
        IfElse(..) | Match(..) | Block(..) => true,
        _ => false,
    }
}

type PString = InternedStr;
type ParseResult<T> = Result<T, String>;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a mut Buffer) -> Parser<'a> {
        Parser {
            lexer: Lexer::new(input),
        }
    }

    fn statement(&mut self) -> ParseResult<(Expr<PString>, bool)> {
        match *self.lexer.peek() {
            TLet => {
                self.lexer.next();
                let id = match *expect!(self, TIdentifier(..)) {
                    TIdentifier(id) => id,
                    _ => fail!(),
                };
                expect!(self, TAssign);
                let expr = self.expression()?;
                expect!(self, TSemicolon);
                Ok((Let(id, Box::new(expr)), true))
            }
            _ => match self.expression() {
                Ok(e) => {
                    if is_statement(&e) {
                        Ok((e, true))
                    } else if matches!(self.lexer.peek(), &TSemicolon) {
                        self.lexer.next();
                        Ok((e, true))
                    } else {
                        Ok((e, false))
                    }
                }
                Err(e) => Err(e),
            },
        }
    }

    fn expression(&mut self) -> ParseResult<Expr<PString>> {
        let e = self.sub_expression()?;
        self.binary_expression(e, 0)
    }
    fn sub_expression(&mut self) -> ParseResult<Expr<PString>> {
        match *self.lexer.next() {
            TIdentifier(id) => Ok(Identifier(id)),
            TOpenParen => {
                let e = self.expression()?;
                expect!(self, TCloseParen);
                Ok(e)
            }
            TOpenBrace => {
                let mut exprs = Vec::new();
                loop {
                    let (expr, is_stm) = self.statement()?;
                    exprs.push(expr);
                    if !is_stm {
                        break;
                    }
                }
                expect!(self, TCloseBrace);
                Ok(Block(exprs))
            }
            TInteger(i) => Ok(Literal(Integer(i))),
            TFloat(f) => Ok(Literal(Float(f))),
            TString(s) => Ok(Literal(TString(s))),
            x => {
                self.lexer.backtrack();
                Err(format!("Token {} does not start an expression", x))
            }
        }
    }

    fn binary_expression(
        &mut self,
        inL: Expr<PString>,
        minPrecedence: i32,
    ) -> ParseResult<Expr<PString>> {
        let mut lhs = inL;
        self.lexer.next();
        loop {
            let lhs_op;
            let lhs_prec;
            match *self.lexer.current() {
                TOperator(op) => {
                    lhs_prec = precedence(op.as_slice());
                    lhs_op = op;
                    if lhs_prec < minPrecedence {
                        break;
                    }
                }
                _ => break,
            };
            debug!("Op {}", lhs_op);

            let mut rhs = self.sub_expression()?;
            self.lexer.next();
            loop {
                let lookahead;
                match *self.lexer.current() {
                    TOperator(op) => {
                        lookahead = precedence(op.as_slice());
                        if lookahead < lhs_prec {
                            break;
                        }
                        debug!("Inner op {}", op);
                    }
                    _ => break,
                }
                self.lexer.backtrack();
                rhs = self.binary_expression(rhs, lookahead);
                self.lexer.next();
            }
            lhs = BinOp(Box::new(lhs), lhs_op.clone(), Box::new(rhs))
        }
        self.lexer.backtrack();
        Ok(lhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::*;
    use interner::*;
    use std::io::BufReader;

    fn binop(l: Expr<InternedStr>, s: &str, r: Expr<InternedStr>) -> Expr<InternedStr> {
        BinOp(Box::new(l), intern(s), Box::new(r))
    }
    fn int(i: i32) -> Expr<InternedStr> {
        Literal(Integer(i))
    }
    fn let_(s: &str, e: Expr<InternedStr>) -> Expr<InternedStr> {
        Let(intern(s), Box::new(e))
    }
    fn id(s: &str) -> Expr<InternedStr> {
        Identifier(intern(s))
    }

    #[test]
    fn operators() {
        let mut buffer = BufReader::new("1 / 4 + (2 - 3) * 2".as_bytes());
        let mut parser = Parser::new(&mut buffer);
        let expr = parser.expression().unwrap_or_else(|err| fail!(err));
        assert_eq!(
            expr,
            binop(
                binop(int(1), "/", int(4)),
                "+",
                binop(binop(int(2), "-", int(3)), "*", int(2))
            )
        );
    }
    #[test]
    fn block() {
        let mut buffer = BufReader::new("1 / { let x = 2; x }".as_bytes());
        let mut parser = Parser::new(&mut buffer);
        let expr = parser.expression().unwrap_or_else(|err| fail!(err));
        assert_eq!(
            expr,
            binop(int(1), "/", Block(vec!(let_("x", int(2)), id("x"))))
        );
    }
}
