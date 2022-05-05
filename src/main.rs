#![feature(globs, phase, macro_rules)]
#[phase(plugin, link)]
mod ast;
mod interner;
mod lexer;
mod parser;

fn main() {}
