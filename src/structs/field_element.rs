extern crate modulo;
use modulo::Mod;
use std::fmt; 
use std::ops::{Add, Mul};
use std::cmp::PartialEq;
use super::gf2::GF2;


pub struct FieldElement {
    n: usize,
    values: Vec<GF2>
}

impl FieldElement {
    pub fn new(n: usize, values: Vec<GF2>) -> Self {
        FieldElement { n, values }
    }
}


impl FieldElement {
    pub fn clone(&self) -> Self {
        let mut new_values = Vec::new();
        for i in 0..self.n {
            new_values.push(self.values[i].clone());
        }
        FieldElement { n: self.n, values: new_values }
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
        FieldElement { n: self.n, values: new_values }
    }
}


fn vec_to_int(values: &Vec<GF2>) -> u32 {
    let mut result = 0;
    for i in 0..values.len() {
        result += values[i].to_int() << i;
    }
    result
}

// implement multiplication
impl<'a> Mul for &'a FieldElement {
    type Output = FieldElement;

    fn mul(self, other: Self) -> FieldElement {
        let lhs = vec_to_int(&self.values);
        let rhs = vec_to_int(&other.values);
        let result = (lhs * rhs).modulo(2_u32.pow(self.n as u32)-1)   ;
        let mut new_values = Vec::new();
        let mut binary = format!("{:b}", result);
        while binary.len() < self.n {
            binary = format!("0{}", binary);
        }
        for c in binary.chars() {
            new_values.push(GF2::new(c.to_digit(10).unwrap() as i8));
        }

        FieldElement { n: self.n, values: new_values }
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



