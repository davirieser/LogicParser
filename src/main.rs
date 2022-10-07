#![allow(unused)]

/* --------------------------------------------------------------------------------------------- */

mod parser;
use parser::ast_node::ASTNode;
use parser::operators::{InfixOperator, UnaryOperator};
use parser::parsing::Parsed;

/* --------------------------------------------------------------------------------------------- */

fn main() {
    match Parsed::new("!!(!!A & B) | (A & B)") {
        Ok(mut parsed) => {
            println!("{}", parsed);
            println!("{:#}", parsed);
            println!("{:b}", parsed);
            println!("{:#b}", parsed);
            println!("{:?}", parsed);
            println!("{:#?}", parsed);

            parsed = parsed.try_simplify();

            println!("{}", parsed);
            println!("{:#}", parsed);
            println!("{:b}", parsed);
            println!("{:#b}", parsed);
            println!("{:?}", parsed);
            println!("{:#?}", parsed);
        }
        Err(s) => print!("{}", s),
    }
}

/* --------------------------------------------------------------------------------------------- */
