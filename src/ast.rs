use interner::InternedStr;

#[derive(Clone, Eq, PartialEq, Show)]
pub enum Type<Id> {
  Type(Id),
  FunctionType(Vec<Type<Id>>, Box<Type<Id>>),
}

#[derive(Clone, PartialEq, Show)]
pub enum Literal {
  Integer(i32),
  Float(f64),
  String(InternedStr),
}

#[derive(Clone, PartialEq, Show)]
pub enum Pattern<Id> {
  Constructor(Id, Vec<Id>),
  IdentifierPattern(Id),
}

#[derive(Clone, PartialEq, Show)]
pub struct Alternative<Id> {
  pub pattern: Pattern<Id>,
  pub expression: Expr<Id>,
}

#[derive(Clone, PartialEq, Show)]
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

#[derive(Clone, PartialEq, Show)]
pub struct Field<Id> {
  pub name: Id,
  pub typ: Type<Id>,
}

#[derive(Clone, PartialEq, Show)]
pub struct Function<Id> {
  pub name: Id,
  pub arguments: Vec<Field<Id>>,
  pub expression: Expr<Id>,
}

#[derive(Clone, PartialEq, Show)]
pub struct Struct<Id> {
  pub fields: Vec<Field<Id>>,
}
