use crate::{diagnostic, int_kind, pass, void_kind};

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum AnyKindNode {
  IntKind(int_kind::IntKind),
  VoidKind(void_kind::VoidKind),
}

pub trait Node {
  fn accept(&mut self, pass: &dyn pass::Pass);

  fn get_children(&self) -> Vec<&dyn Node> {
    vec![]
  }

  fn get_diagnostics(&self) -> Vec<diagnostic::Diagnostic> {
    vec![]
  }
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Identifier {
  name: String,
}
