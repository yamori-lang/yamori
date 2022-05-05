#![feature(globs, phase, macro_rules)]
extern crate collections;
#[phase(plugin, link)]
extern crate log;

mod ast;
mod interner;
mod lexer;
mod parser;

fn main() {}
