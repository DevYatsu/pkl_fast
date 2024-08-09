use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "pkl.pest"]
pub struct PklParser;
