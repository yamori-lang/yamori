use crate::{block, node, pass, prototype};

#[derive(Hash, Eq, PartialEq, Debug)]

pub struct Function<'a> {
  prototype: prototype::Prototype<'a>,
  body: block::Block,
}

impl<'a> node::Node for Function<'a> {
  fn accept(&mut self, pass: &dyn pass::Pass) {
    // pass.visit_function(self);
  }
}
