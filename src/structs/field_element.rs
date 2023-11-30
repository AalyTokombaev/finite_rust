extern crate modulo;
use modulo::Mod;
use std::fmt;
use std::ops::{Add, Mul};
use std::cmp::PartialEq;
use super::gf2::GF2;
use super::field::Field;

pub struct FieldElement {
    n: usize,
    values: Vec<GF2>,
    primitive_poly: Vec<GF2>,
}

impl FieldElement {
    pub fn new(n: usize, values: Vec<GF2>, primitive_poly: Vec<GF2>) -> Self {
        FieldElement { n, values, primitive_poly }
    }

    pub fn clone(&self) -> Self {
        let mut new_values = Vec::new();
        for i in 0..self.n {
            new_values.push(self.values[i].clone());
        }
        FieldElement { n: self.n, values: new_values, primitive_poly: self.primitive_poly.clone() }
    }
}

// implement addition
impl<'a> Add for &'a FieldElement {
    type Output = FieldElement;

    fn add(self, other: Self) -> FieldElement {
        let mut new_values = Vec::new();
        for i in 0..self.n {
            new_values.push(self.values[i] + other.values[i]);
        }
        FieldElement { n: self.n, values: new_values, primitive_poly: self.primitive_poly.clone() }
    }
}

// implement multiplication
impl<'a> Mul for &'a FieldElement {
    type Output = FieldElement;

    fn mul(self, other: Self) -> FieldElement {
        let mut result = FieldElement::new(self.n, vec![GF2::new(0); self.n], self.primitive_poly.clone());
        for i in 0..self.n {
            for j in 0..self.n {
                if i + j < self.n {
                    result.values[i + j] = result.values[i + j] + self.values[i] * other.values[j];
                } else {
                    result.values[i + j - self.n] = result.values[i + j - self.n] + self.values[i] * other.values[j];
                }
            }
        }
        FieldElement { n: self.n, values: self.values.clone(), primitive_poly: self.primitive_poly.clone() }
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..self.n {
            if self.values[i] != other.values[i] {
                return false;
            }
        }
        true
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        s.push('[');
        for i in 0..self.n {
            s.push_str(&format!(" {} ", self.values[i]));
        }
        s.push(']');
        write!(f, "{}", s)
    }
}

fn vec_to_int(values: &Vec<GF2>) -> u32 {
    let mut result = 0;
    for i in 0..values.len() {
        result += values[i].to_int() << i;
    }
    result
}
