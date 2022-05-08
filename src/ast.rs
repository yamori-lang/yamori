use crate::interner::InternedStr;

#[derive(Clone, Eq, PartialEq)]
pub enum Type<Id> {
  Type(Id),
  FunctionType(Vec<Type<Id>>, Box<Type<Id>>),
}

#[derive(Clone, PartialEq)]
pub enum Literal {
  Integer(i32),
  Float(f64),
  String(InternedStr),
}

#[derive(Clone, PartialEq)]
pub enum Pattern<Id> {
  Constructor(Id, Vec<Id>),
  IdentifierPattern(Id),
}

#[derive(Clone, PartialEq)]
pub struct Alternative<Id> {
  pub pattern: Pattern<Id>,
  pub expression: Expr<Id>,
}

#[derive(Clone, PartialEq)]
pub enum Expr<Id> {
  Identifier(Id),
  Literal(Literal),
  Call(Id, Vec<Expr<Id>>),
  IfElse(Box<Expr<Id>>, Box<Expr<Id>>, Box<Expr<Id>>),
  Match(Box<Expr<Id>>, Vec<Alternative<Id>>),
  Block(Vec<Expr<Id>>),
  BinOp(Id, Box<Expr<Id>>),
  Let(Id, Box<Expr<Id>>),
}

#[derive(Clone, PartialEq)]
pub struct Field<Id> {
  pub name: Id,
  pub typ: Type<Id>,
}

#[derive(Clone, PartialEq)]
pub struct Function<Id> {
  pub name: Id,
  pub arguments: Vec<Field<Id>>,
  pub expression: Expr<Id>,
}

#[derive(Clone, PartialEq)]
pub struct Struct<Id> {
  pub fields: Vec<Field<Id>>,
}
