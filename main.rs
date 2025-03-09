extern crate lalrpop_util;

mod xybercode {
    lalrpop_mod!(xybercode);
}

use xybercode::xybercode::ProgramParser;
use std::fs;

#[derive(Debug)]
enum Statement {
    Command(String),
    Task(String),
    Expectation(String),
    Instruction(String),
    Execution(String, Expression),
    Block(Vec<Statement>),
}

#[derive(Debug)]
enum Expression {
    Number(i64),
    Text(String),
    Boolean(bool),
    Identifier(String),
    Add(Box<Expression>, Box<Expression>),
}

fn main() {
    let code = fs::read_to_string("example.xyber").expect("Unable to read file");
    
    match ProgramParser::new().parse(&code) {
        Ok(parsed) => println!("{:#?}", parsed),
        Err(e) => eprintln!("Parsing error: {:?}", e),
    }
}
