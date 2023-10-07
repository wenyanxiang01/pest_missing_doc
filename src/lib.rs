//!
#![deny(missing_docs)]
#![deny(unreachable_pub)]

use pest_derive::Parser;

pub(crate) mod parser;

/*
error: missing documentation for an associated function
  --> src\lib.rs:10:10
   |
10 | #[derive(Parser)]
   |          ^^^^^^
   |
note: the lint level is defined here
  --> src\lib.rs:2:9
   |
2  | #![deny(missing_docs)]
   |         ^^^^^^^^^^^^
   = note: this error originates in the derive macro `Parser` (in Nightly builds, run with -Z macro-backtrace for more info)
*/
/// missing docs here
#[derive(Parser)]
#[grammar = "test.pest"]
struct Parser;
