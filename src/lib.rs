use pest_derive::Parser;
use pest::{error::Error, iterators::Pairs, Parser};

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

pub fn parse_markdown(input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    return Grammar::parse(Rule::markdown, input);
}

pub fn parse_by_rule(rule: Rule, input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    return Grammar::parse(rule, input);
}