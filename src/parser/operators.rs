#![allow(unused)]

/* --------------------------------------------------------------------------------------------- */

use crate::parser::ast_node::ASTNode;
use std::fmt::{Debug, Display, Formatter};

/* --------------------------------------------------------------------------------------------- */

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Assoc {
    Right,
    Left,
    NotApplicable,
}

trait Operator {
    fn assoc(&self) -> Assoc {
        Assoc::NotApplicable
    }
    fn precedence(&self) -> usize {
        0
    }
}

/* --------------------------------------------------------------------------------------------- */

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum UnaryOperator {
    Not,
}

impl UnaryOperator {
    pub fn apply(&self, value: bool) -> bool {
        match self {
            UnaryOperator::Not => !value,
            _ => unreachable!(),
        }
    }
}

impl Operator for UnaryOperator {
    fn assoc(&self) -> Assoc {
        Assoc::NotApplicable
    }
    fn precedence(&self) -> usize {
        0
    }
}

impl Debug for UnaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOperator::Not => write!(f, "Not"),
            _ => unreachable!(),
        }
    }
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOperator::Not => write!(f, "!"),
            _ => unreachable!(),
        }
    }
}

/* --------------------------------------------------------------------------------------------- */

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InfixOperator {
    And,
    Or,
    Implication,
}

impl InfixOperator {
    pub fn apply(&self, lhs: bool, rhs: bool) -> bool {
        match self {
            InfixOperator::And => lhs & rhs,
            InfixOperator::Or => lhs | rhs,
            InfixOperator::Implication => (!lhs) | rhs,
            _ => unreachable!(),
        }
    }
    pub fn simplify(self, lhs: ASTNode, rhs: ASTNode) -> ASTNode {
        let lhs = lhs.try_simplify();
        let rhs = rhs.try_simplify();

        match (lhs, rhs) {
            (ASTNode::Literal(l1), ASTNode::Literal(l2)) => ASTNode::Literal(self.apply(l1, l2)),
            (a1, ASTNode::Literal(l2)) => match (self, l2) {
                (InfixOperator::And, true)
                | (InfixOperator::Or, false)
                | (InfixOperator::Implication, false) => a1,
                (InfixOperator::And, false) => ASTNode::Literal(false),
                (InfixOperator::Or, true) | (InfixOperator::Implication, true) => {
                    ASTNode::Literal(true)
                }
            },
            (ASTNode::Literal(l1), a2) => match (self, l1) {
                (InfixOperator::And, true)
                | (InfixOperator::Or, false)
                | (InfixOperator::Implication, true) => a2,
                (InfixOperator::And, false) => ASTNode::Literal(false),
                (InfixOperator::Or, true) | (InfixOperator::Implication, false) => {
                    ASTNode::Literal(true)
                }
            },
            (ASTNode::Identifier(i1), ASTNode::Identifier(i2)) => {
                if (i1 == i2) {
                    match self {
                        InfixOperator::And => ASTNode::Identifier(i1),
                        InfixOperator::Or | InfixOperator::Implication => ASTNode::Literal(true),
                    }
                } else {
                    ASTNode::InfixOperation {
                        op: self,
                        lhs: Box::new(ASTNode::Identifier(i1)),
                        rhs: Box::new(ASTNode::Identifier(i2)),
                    }
                }
            }
            (lhs, rhs) => ASTNode::InfixOperation {
                op: self,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
        }
    }
}

/* --------------------------------------------------------------------------------------------- */

impl Operator for InfixOperator {
    fn assoc(&self) -> Assoc {
        match self {
            InfixOperator::And | InfixOperator::Or => Assoc::Left,
            InfixOperator::Implication => Assoc::Right,
            _ => unreachable!(),
        }
    }
    fn precedence(&self) -> usize {
        match self {
            InfixOperator::And | InfixOperator::Or => 2,
            InfixOperator::Implication => 1,
            _ => unreachable!(),
        }
    }
}

/* --------------------------------------------------------------------------------------------- */

impl Display for InfixOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InfixOperator::And => write!(f, "&"),
            InfixOperator::Or => write!(f, "|"),
            InfixOperator::Implication => write!(f, "->"),
            _ => unreachable!(),
        }
    }
}

/* --------------------------------------------------------------------------------------------- */
