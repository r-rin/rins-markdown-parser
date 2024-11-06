use pest_derive::Parser;
use pest::{error::Error, Parser};

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

fn main() {
   println!("Hello world!")
}
