//!
#![deny(missing_docs)]

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "test.pest"]
struct Parser;
