#![allow(dead_code, unused)]

use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Index, IndexMut, Not, Shr, ShrAssign};

#[derive(Debug, Clone, Copy)]
pub enum TruthValue {
    DontCare,
    True,
    False,
}

impl From<TruthValue> for bool {
    fn from(t: TruthValue) -> Self {
        match t {
            TruthValue::DontCare | TruthValue::True => true,
            _ => false,
        }
    }
}

impl BitAnd for TruthValue {
    type Output = TruthValue;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (TruthValue::True, other) | (other, TruthValue::True) => other,
            (TruthValue::False, other) | (other, TruthValue::False) => TruthValue::False,
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
            (TruthValue::False, other) | (other, TruthValue::False) => other,
            (TruthValue::True, other) | (other, TruthValue::True) => TruthValue::True,
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
            (TruthValue::False, _) => TruthValue::True,
            (_, TruthValue::True) => TruthValue::True,
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
