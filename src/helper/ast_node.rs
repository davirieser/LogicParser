#![allow(unused)]

/* --------------------------------------------------------------------------------------------- */

use crate::helper::operators::{InfixOperator, UnaryOperator};

use std::fmt::{Binary, Debug, Display, Formatter};

/* --------------------------------------------------------------------------------------------- */

#[derive(Debug, Clone)]
pub enum ASTNode {
    InfixOperation {
        op: InfixOperator,
        lhs: Box<ASTNode>,
        rhs: Box<ASTNode>,
    },
    UnaryOperation {
        op: UnaryOperator,
        expr: Box<ASTNode>,
    },
    Identifier(usize),
    Literal(bool),
}

/* --------------------------------------------------------------------------------------------- */

impl ASTNode {
    pub fn negate(self) -> Self {
        match self {
            ASTNode::Literal(v) => ASTNode::Literal(!v),
            ASTNode::UnaryOperation { op, expr } if op == UnaryOperator::Not => *expr,
            expr => ASTNode::UnaryOperation {
                op: UnaryOperator::Not,
                expr: Box::new(expr),
            },
        }
    }
    pub fn evaluate(&self, binding: &Vec<(usize, bool)>) -> bool {
        match self {
            Self::Literal(l) => *l,
            Self::Identifier(i) => binding[binding.binary_search_by(|e| e.0.cmp(i)).unwrap()].1,
            Self::UnaryOperation { op, expr } => op.apply(expr.evaluate(binding)),
            Self::InfixOperation { op, lhs, rhs } => {
                op.apply(lhs.evaluate(binding), rhs.evaluate(binding))
            }
            _ => unreachable!(),
        }
    }
    pub fn try_simplify(self) -> Self {
        match self {
            Self::UnaryOperation { op, expr } => match expr.try_simplify() {
                Self::Literal(l) => Self::Literal(op.apply(l)),
                // Remove double Negations
                Self::UnaryOperation { op: op2, expr }
                    if (op == UnaryOperator::Not) && (op2 == UnaryOperator::Not) =>
                {
                    *expr
                }
                e => Self::UnaryOperation {
                    op,
                    expr: Box::new(e),
                },
            },
            Self::InfixOperation { op, lhs, rhs } => {
                op.simplify(lhs.try_simplify(), rhs.try_simplify())
            }
            _ => self,
        }
    }
}

/* --------------------------------------------------------------------------------------------- */

/// Prints the Expression with Braces.
impl Binary for ASTNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ASTNode::InfixOperation { op, lhs, rhs } => {
                write!(f, "{:b} {} {:b}", lhs.as_ref(), op, rhs.as_ref())
            }
            ASTNode::UnaryOperation { op, expr } => write!(f, "{}{:b}", op, expr.as_ref()),
            ASTNode::Identifier(i) => write!(f, "{}", i),
            ASTNode::Literal(l) => write!(f, "{}", l),
            _ => unreachable!(),
        }
    }
}

/// Prints the Expression without Braces.
impl Display for ASTNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ASTNode::InfixOperation { op, lhs, rhs } => write!(f, "({} {} {})", lhs, op, rhs),
            ASTNode::UnaryOperation { op, expr } => write!(f, "{}{}", op, expr.as_ref()),
            ASTNode::Identifier(i) => write!(f, "{}", i),
            ASTNode::Literal(l) => write!(f, "{}", l),
            _ => unreachable!(),
        }
    }
}

/* --------------------------------------------------------------------------------------------- */

impl ASTNode {
    pub fn fmt_binary_names<'a>(
        &self,
        bindings: &Vec<&'a str>,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            ASTNode::InfixOperation { op, lhs, rhs } => {
                write!(f, "(");
                lhs.fmt_display_names(bindings, f);
                write!(f, "{}", op);
                rhs.fmt_display_names(bindings, f);
                write!(f, ")")
            }
            ASTNode::UnaryOperation { op, expr } => {
                write!(f, "{}", op);
                expr.fmt_display_names(bindings, f)
            }
            ASTNode::Identifier(i) => write!(f, "{}", bindings[*i]),
            ASTNode::Literal(l) => write!(f, "{}", l),
            _ => unreachable!(),
        }
    }
}

impl ASTNode {
    pub fn fmt_debug_names<'a>(
        &self,
        bindings: &Vec<&'a str>,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        // TODO: Somehow make this be indented?
        match self {
            ASTNode::InfixOperation { op, lhs, rhs } => {
                write!(f, "InfixOperation {{op: {:?}, lhs: ", op);
                lhs.fmt_debug_names(bindings, f);
                write!(f, ", rhs: ");
                rhs.fmt_debug_names(bindings, f);
                write!(f, "}}")
            }
            ASTNode::UnaryOperation { op, expr } => {
                write!(f, "{:?}", op);
                expr.fmt_debug_names(bindings, f)
            }
            ASTNode::Identifier(i) => write!(f, "\"{}\"", bindings[*i]),
            ASTNode::Literal(l) => write!(f, "{}", l),
            _ => unreachable!(),
        }
    }
}

impl ASTNode {
    /// Print the Expression to the specified Formatter using the
    /// provided Bindings from Integer-Identifier to Identifier-Name.
    pub fn fmt_display_names<'a>(
        &self,
        bindings: &Vec<&'a str>,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            ASTNode::InfixOperation { op, lhs, rhs } => {
                if (f.alternate()) {
                    lhs.fmt_display_names(bindings, f);
                    write!(f, " {} ", op);
                    rhs.fmt_display_names(bindings, f)
                } else {
                    write!(f, "(");
                    lhs.fmt_display_names(bindings, f);
                    write!(f, " {} ", op);
                    rhs.fmt_display_names(bindings, f);
                    write!(f, ")")
                }
            }
            ASTNode::UnaryOperation { op, expr } => {
                write!(f, "{}", op);
                expr.fmt_display_names(bindings, f)
            }
            ASTNode::Identifier(i) => write!(f, "{}", bindings[*i]),
            ASTNode::Literal(l) => write!(f, "{}", l),
            _ => unreachable!(),
        }
    }
}

/* --------------------------------------------------------------------------------------------- */
