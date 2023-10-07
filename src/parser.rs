use pest_derive::Parser;

/*
The `pub enum Rule` here should theoretically be detected by `#![deny(unreachable_pub)]`.
*/
#[derive(Parser)]
#[grammar = "test.pest"]
pub(crate) struct Parser;

/*
error: missing documentation for an enum
  --> src\lib.rs:14:1
   |
14 | pub enum Rul {}
   | ^^^^^^^^^^^^
*/
pub enum Rul {}
