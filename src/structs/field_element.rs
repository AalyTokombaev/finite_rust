use std::fmt; 
use std::ops::Add;
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
impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut new_values = Vec::new();
        for i in 0..self.n {
            new_values.push(self.values[i] + other.values[i]);
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
        s.push("[");
        for i in 0..self.n {
            s.push_str(&format!(" {} ", self.values[i]));
        }
        s.push("[");
        write!(f, "{}", s)
    }
}



