#![allow(unused)]

/* --------------------------------------------------------------------------------------------- */

mod helper;
use helper::ast_node::ASTNode;
use helper::operators::{InfixOperator, UnaryOperator};
use helper::parsing::Parsed;

/* --------------------------------------------------------------------------------------------- */

fn main() {
    match Parsed::new("(!!A & B) | (A & B)") {
        Ok(mut parsed) => {
            println!("{:}", parsed);

            parsed = parsed.try_simplify();

            println!("{:}", parsed);
            println!("{:b}", parsed);
            println!("{:?}", parsed);
        }
        Err(s) => print!("{}", s),
    }
}

/* --------------------------------------------------------------------------------------------- */
