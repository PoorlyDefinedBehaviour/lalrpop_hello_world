pub mod ast;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

fn main() {
  let mut errors = Vec::new();

  let expr = grammar::ExprParser::new().parse(2, &mut errors, "2 + 2, * 3");

  dbg!(&errors);

  dbg!(&expr);
}
