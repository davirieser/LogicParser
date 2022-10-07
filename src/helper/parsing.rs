#![allow(unused)]

/* --------------------------------------------------------------------------------------------- */

use crate::helper::{
    ast_node::ASTNode,
    operators::{InfixOperator, UnaryOperator},
};

/* --------------------------------------------------------------------------------------------- */

use pest::iterators::{Pair, Pairs};
use pest::Parser as ParserTrait;
use pest_derive::Parser;

/* --------------------------------------------------------------------------------------------- */

use std::collections::HashMap;
use std::fmt::{Binary, Debug, Display, Formatter};

/* --------------------------------------------------------------------------------------------- */

#[derive(Parser)]
#[grammar = "my_grammar.pest"]
struct Parser;

/* --------------------------------------------------------------------------------------------- */

#[derive(Clone)]
pub struct Parsed<'a> {
    expression: ASTNode,
    var_bindings: Vec<&'a str>,
}

/* --------------------------------------------------------------------------------------------- */

impl<'a> Parsed<'a> {
    pub fn new(s: &'a str) -> Result<Self, String> {
        let mut num_variables: usize = 0;
        // Create local HashMap to quickly check if Identifiers were already known.
        let mut set: HashMap<&'a str, usize> = HashMap::new();
        // Store Identifiers in Vector for easier and faster access.
        let mut map: Vec<&'a str> = Vec::new();

        // Create Closure which maps the Identifier to the Binding.
        let mut insert_variable = |var: &'a str| match set.get(var) {
            // If Identifier is already known return it's Index in the Vector
            Some(idx) => *idx,
            // If the Identifier is not known add it to the Map and Vector and return
            // the corresponding Index.
            None => {
                set.insert(var, num_variables);
                map.push(var);
                num_variables += 1;
                num_variables - 1
            }
        };

        // Try matching the Top-Rule
        match Parser::parse(Rule::_start, s) {
            Ok(mut pairs) => {
                let pair = pairs.next().unwrap();
                let parsed = parse_expression(pair, &mut insert_variable);
                // Free the Set because all Variables are bound/mapped.
                drop(set);
                Ok(Parsed {
                    expression: parsed,
                    var_bindings: map,
                })
            }
            // Return the Pest-Error if something went wrong
            Err(e) => Err(e.to_string()),
        }
    }
    pub fn num_variables(&self) -> usize {
        self.var_bindings.len()
    }
    pub fn evaluate(&self, binding: &Vec<(usize, bool)>) -> bool {
        self.expression.evaluate(binding)
    }
    pub fn try_simplify(mut self) -> Self {
        // Recursively simplify Expressions
        self.expression = self.expression.try_simplify();
        self
    }
}

/* --------------------------------------------------------------------------------------------- */

/// Prints the Expression with Braces.
impl<'a> Binary for Parsed<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            self.expression.fmt_binary_names(&self.var_bindings, f)
        } else {
            write!(f, "{:b}", self.expression)
        }
    }
}

impl<'a> Debug for Parsed<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            self.expression.fmt_debug_names(&self.var_bindings, f)
        } else {
            write!(f, "{:#?}", self.expression)
        }
    }
}

/// Prints the Expression without Braces.
impl<'a> Display for Parsed<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            self.expression.fmt_display_names(&self.var_bindings, f)
        } else {
            write!(f, "{}", self.expression)
        }
    }
}

/* --------------------------------------------------------------------------------------------- */

fn parse_expression<'a, F>(pair: Pair<'a, Rule>, i: &mut F) -> ASTNode
where
    F: FnMut(&'a str) -> usize,
{
    match pair.as_rule() {
        Rule::Expression => parse_expression(pair.into_inner().next().unwrap(), i),
        Rule::PrimaryExpression => {
            let inner = pair.into_inner().next().unwrap();
            match inner.as_rule() {
                Rule::Expression => parse_expression(inner, i),
                Rule::Variable => parse_variable(inner, i),
                _ => unreachable!(),
            }
        }
        Rule::ImplicationExpression => parse_right_associtiv(pair.into_inner(), i),
        Rule::AndOrExpression => parse_left_associativ(pair.into_inner(), i),
        Rule::UnaryExpression => parse_unary_expression(pair.into_inner(), i),
        Rule::Variable => parse_variable(pair.into_inner().next().unwrap(), i),
        _ => unreachable!(),
    }
}

/* --------------------------------------------------------------------------------------------- */

fn parse_infix_operator<'a, F>(op: &Pair<Rule>, i: &mut F) -> InfixOperator
where
    F: FnMut(&'a str) -> usize,
{
    match op.as_str() {
        "&" | "*" => InfixOperator::And,
        "|" | "+" => InfixOperator::Or,
        "->" => InfixOperator::Implication,
        _ => {
            // Use eq_ignore_ascii_case to avoid unnecessary Allocations.
            if (op.as_str().eq_ignore_ascii_case("and")) {
                InfixOperator::And
            } else if (op.as_str().eq_ignore_ascii_case("or")) {
                InfixOperator::Or
            } else {
                unreachable!()
            }
        }
    }
}

/* --------------------------------------------------------------------------------------------- */

fn parse_right_associtiv<'a, F>(mut pair: Pairs<'a, Rule>, i: &mut F) -> ASTNode
where
    F: FnMut(&'a str) -> usize,
{
    let mut lhs = parse_expression(pair.next().unwrap(), i);
    match pair.peek() {
        Some(_) => _parse_right_associtiv(lhs, pair, i),
        None => lhs,
    }
}

fn _parse_right_associtiv<'a, F>(lhs: ASTNode, mut pair: Pairs<'a, Rule>, i: &mut F) -> ASTNode
where
    F: FnMut(&'a str) -> usize,
{
    let mut lhs = lhs;
    match (pair.next(), pair.next()) {
        (Some(op), Some(rhs)) => ASTNode::InfixOperation {
            op: parse_infix_operator(&op, i),
            lhs: Box::new(lhs),
            rhs: Box::new(_parse_right_associtiv(parse_expression(rhs, i), pair, i)),
        },
        _ => lhs,
    }
}

/* --------------------------------------------------------------------------------------------- */

fn parse_left_associativ<'a, F>(mut pair: Pairs<'a, Rule>, i: &mut F) -> ASTNode
where
    F: FnMut(&'a str) -> usize,
{
    let mut lhs = parse_expression(pair.next().unwrap(), i);
    match pair.peek() {
        Some(_) => _parse_left_associativ(lhs, pair, i),
        None => lhs,
    }
}

fn _parse_left_associativ<'a, F>(lhs: ASTNode, mut pair: Pairs<'a, Rule>, i: &mut F) -> ASTNode
where
    F: FnMut(&'a str) -> usize,
{
    let mut lhs = lhs;
    while let (Some(op), Some(rhs)) = (pair.next(), pair.next()) {
        lhs = ASTNode::InfixOperation {
            op: parse_infix_operator(&op, i),
            lhs: Box::new(lhs),
            rhs: Box::new(parse_expression(rhs, i)),
        };
    }
    lhs
}

/* --------------------------------------------------------------------------------------------- */

fn parse_unary_operator(op: &Pair<Rule>) -> UnaryOperator {
    match op.as_str() {
        "!" | "~" => UnaryOperator::Not,
        _ => unreachable!(),
    }
}

fn parse_unary_expression<'a, F>(mut pair: Pairs<'a, Rule>, i: &mut F) -> ASTNode
where
    F: FnMut(&'a str) -> usize,
{
    match (pair.next(), pair.next()) {
        (Some(op), Some(expr)) => ASTNode::UnaryOperation {
            op: parse_unary_operator(&op),
            expr: Box::new(parse_expression(expr, i)),
        },
        (Some(expr), None) => parse_expression(expr, i),
        _ => unreachable!(),
    }
}

/* --------------------------------------------------------------------------------------------- */

fn parse_variable<'a, F>(pair: Pair<'a, Rule>, i: &mut F) -> ASTNode
where
    F: FnMut(&'a str) -> usize,
{
    match pair.as_rule() {
        Rule::Literal => {
            // Use eq_ignore_ascii_case to avoid unnecessary Allocations
            if pair.as_str().eq_ignore_ascii_case("true") {
                ASTNode::Literal(true)
            } else if pair.as_str().eq_ignore_ascii_case("false") {
                ASTNode::Literal(false)
            } else {
                unreachable!()
            }
        }
        Rule::Identifier => {
            // Pass String-Identifier up to Parent and get Int-Identifier back
            let id = i(pair.as_str());
            ASTNode::Identifier(id)
        }
        Rule::Variable => parse_variable(pair.into_inner().next().unwrap(), i),
        _ => unreachable!(),
    }
}

/* --------------------------------------------------------------------------------------------- */
