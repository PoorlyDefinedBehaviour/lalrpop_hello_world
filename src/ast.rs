#[derive(Debug)]
pub enum Opcode {
  Mul,
  Div,
  Add,
  Sub,
}

#[derive(Debug)]
pub enum Expr {
  Number(i32),
  Op(Box<Expr>, Opcode, Box<Expr>),
  Error,
}
