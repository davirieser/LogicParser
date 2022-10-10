#![allow(dead_code, unused)]

use std::fmt::{Debug, Display, Formatter};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Index, IndexMut, Not, Shr, ShrAssign};

#[derive(Debug, Clone, Copy)]
pub enum TruthValue {
    DontCare,
    True,
    False,
}

impl From<TruthValue> for bool {
    fn from(t: TruthValue) -> Self {
        matches!(t, TruthValue::DontCare | TruthValue::True)
    }
}

impl From<bool> for TruthValue {
    fn from(b: bool) -> TruthValue {
        match b {
            true => TruthValue::True,
            false => TruthValue::False,
        }
    }
}

impl Display for TruthValue {
    fn fmt(&self, f: Formatter) -> std::fmt::Result {
        match self {
            TruthValue::DontCare => write!(f, "X"),
            TruthValue::False => write!(f, "F"),
            TruthValue::True => write!(f, "T"),
        }
    }
}

impl BitAnd for TruthValue {
    type Output = TruthValue;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (TruthValue::False, _) | (_, TruthValue::False) => TruthValue::False,
            (TruthValue::True, other) | (other, TruthValue::True) => other,
            (_, _) => TruthValue::DontCare,
        }
    }
}

impl BitAndAssign for TruthValue {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}
impl BitOr for TruthValue {
    type Output = TruthValue;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (TruthValue::True, _) | (_, TruthValue::True) => TruthValue::True,
            (TruthValue::False, other) | (other, TruthValue::False) => other,
            (_, _) => TruthValue::DontCare,
        }
    }
}

impl BitOrAssign for TruthValue {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl Not for TruthValue {
    type Output = TruthValue;

    fn not(self) -> Self::Output {
        match self {
            TruthValue::True => TruthValue::False,
            TruthValue::False => TruthValue::True,
            TruthValue::DontCare => TruthValue::DontCare,
        }
    }
}

/// Implication Operator "->"
impl Shr for TruthValue {
    type Output = TruthValue;

    fn shr(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (TruthValue::False, _) | (_, TruthValue::True) => TruthValue::True,
            (TruthValue::True, other) => other,
            (_, _) => TruthValue::DontCare,
        }
    }
}

impl ShrAssign for TruthValue {
    fn shr_assign(&mut self, rhs: Self) {
        *self = *self >> rhs;
    }
}

pub struct Minterm {
    v: Vec<TruthValue>,
}

impl From<Vec<TruthValue>> for Minterm {
    fn from(v: Vec<TruthValue>) -> Self {
        Self { v }
    }
}

impl BitAnd for Minterm {
    type Output = Minterm;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            v: self
                .v
                .into_iter()
                .zip(rhs.v.into_iter())
                .map(|(t1, t2)| t1 & t2)
                .collect(),
        }
    }
}

impl BitAndAssign for Minterm {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = Self {
            v: self
                .v
                .iter()
                .zip(rhs.v.into_iter())
                .map(|(t1, t2)| *t1 & t2)
                .collect(),
        }
    }
}

impl BitOr for Minterm {
    type Output = Minterm;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            v: self
                .v
                .into_iter()
                .zip(rhs.v.into_iter())
                .map(|(t1, t2)| t1 | t2)
                .collect(),
        }
    }
}

impl BitOrAssign for Minterm {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = Self {
            v: self
                .v
                .iter()
                .zip(rhs.v.into_iter())
                .map(|(t1, t2)| *t1 | t2)
                .collect(),
        }
    }
}

impl Not for Minterm {
    type Output = Minterm;

    fn not(self) -> Self::Output {
        Self {
            v: self.v.into_iter().map(|t| !t).collect(),
        }
    }
}

/// Implication Operator "->"
impl Shr for Minterm {
    type Output = Minterm;

    fn shr(self, rhs: Self) -> Self::Output {
        Self {
            v: self
                .v
                .into_iter()
                .zip(rhs.v.into_iter())
                .map(|(t1, t2)| t1 >> t2)
                .collect(),
        }
    }
}

impl ShrAssign for Minterm {
    fn shr_assign(&mut self, rhs: Self) {
        *self = Self {
            v: self
                .v
                .iter()
                .zip(rhs.v.into_iter())
                .map(|(t1, t2)| *t1 >> t2)
                .collect(),
        }
    }
}

impl Index<usize> for Minterm {
    type Output = TruthValue;

    fn index(&self, index: usize) -> &Self::Output {
        &self.v[index]
    }
}

#[cfg(test)]
impl IndexMut<usize> for Minterm {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.v[index]
    }
}
