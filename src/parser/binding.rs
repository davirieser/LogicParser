#![allow(unused)]

use std::fmt::Display;
use std::ops::{Deref, DerefMut, Index, Not};

/* --------------------------------------------------------------------------------------------- */

#[derive(Debug, Clone, Hash)]
pub struct Bindings<'a> {
    v: Vec<bool>,
    n: Vec<&'a str>,
}

/* --------------------------------------------------------------------------------------------- */

impl<'a> Bindings<'a> {
    pub fn new() -> Self {
        Self {
            v: Vec::new(),
            n: Vec::new(),
        }
    }
    pub fn push_binding(&mut self, value: bool, name: &'a str) {
        self.n.push(name);
        self.v.push(value);
    }
    pub fn contains(&self, name: &'a str) -> bool {
        self.n.iter().any(|n| (*n).eq(name))
    }
    pub fn num_bindings(&self) -> usize {
        self.v.len()
    }
    pub fn get_name(&self, idx: usize) -> &'a str {
        self.n[idx]
    }
    pub fn reset_values(&mut self) {
        for i in 0..self.v.len() {
            self.v[i] = false;
        }
    }
    pub fn advance(&mut self) -> Option<&Vec<bool>> {
        match self.next() {
            Some(_) => Some(&self.v),
            None => None,
        }
    }
}

/* --------------------------------------------------------------------------------------------- */

impl<'a> Not for Bindings<'a> {
    type Output = Vec<bool>;

    fn not(self) -> Self::Output {
        self.v.iter().map(|b| !b).collect()
    }
}

impl<'a> Index<usize> for Bindings<'a> {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.v[index]
    }
}

impl<'a> Deref for Bindings<'a> {
    type Target = Vec<bool>;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

impl<'a> Default for Bindings<'a> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
impl<'a> DerefMut for Bindings<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}

/* --------------------------------------------------------------------------------------------- */

impl<'a> Iterator for Bindings<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        for i in (0..self.v.len()).rev() {
            if (!self.v[i]) {
                self.v[i] = !self.v[i];
                return Some(i);
            }
            self.v[i] = !self.v[i];
        }
        // All Truth-Values are reset to false here
        // Basically the Iterator is exhausted here but it can be automatically resumed.
        // This Iterator doesn't return the all false Vec at the next Call though.
        None
    }
}

/* --------------------------------------------------------------------------------------------- */

impl<'a> Display for Bindings<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buff = String::with_capacity((self.v.len() * 3) - 2);
        for i in 0..(self.v.len() - 1) {
            buff.push(if self.v[i] { '1' } else { '0' });
            buff.push(','); buff.push(' ');
        }
        buff.push(if self.v[self.v.len() - 1] { '1' } else { '0' });
        write!(f, "{}", buff)
    }
}

/* --------------------------------------------------------------------------------------------- */

impl<'a> From<Vec<&'a str>> for Bindings<'a> {
    fn from(v: Vec<&'a str>) -> Self {
        let values = vec![false; v.len()];
        let expired = v.is_empty();
        Self { v: values, n: v }
    }
}

impl<'a> From<Vec<(bool, &'a str)>> for Bindings<'a> {
    fn from(v: Vec<(bool, &'a str)>) -> Self {
        let mut values = Vec::with_capacity(v.len());
        let mut names = Vec::with_capacity(v.len());
        let expired = v.is_empty();
        for (v, n) in v {
            values.push(v);
            names.push(n);
        }
        Self {
            v: values,
            n: names,
        }
    }
}

impl<'a> From<Vec<(&'a str, bool)>> for Bindings<'a> {
    fn from(v: Vec<(&'a str, bool)>) -> Self {
        let mut values = Vec::with_capacity(v.len());
        let mut names = Vec::with_capacity(v.len());
        let expired = v.is_empty();
        for (n, v) in v {
            values.push(v);
            names.push(n);
        }
        Self {
            v: values,
            n: names,
        }
    }
}

/* --------------------------------------------------------------------------------------------- */
