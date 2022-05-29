use crate::{block, node, pass, prototype};

#[derive(Hash, Eq, PartialEq, Debug)]

pub struct Function {
  pub is_public: bool,
  pub prototype: prototype::Prototype,
  body: block::Block,
}

impl node::Node for Function {
  fn accept(&mut self, pass: &dyn pass::Pass) {
    // pass.visit_function(self);
  }
}
