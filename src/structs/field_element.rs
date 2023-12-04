extern crate modulo;
use std::fmt;
use std::ops::{Add, Mul};
use std::cmp::PartialEq;
use super::gf2::GF2;

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

impl<'a> Mul for &'a FieldElement {
    type Output = FieldElement;


    
    fn mul(self, b: Self) -> FieldElement {
        let _primitive_bits = vec_to_int(&self.primitive_poly);
        // represent primitive poly as a bit vector
        let mut primitive_bits = Vec::new();
        let mut temp = vec_to_int(&self.primitive_poly);
        while temp > 0 {
            primitive_bits.push(temp % 2);
            temp = temp / 2;
        }
    
        println!("primitive_bits: {:?}", primitive_bits);

        let mut result: i8 = 0b0;
        let mut b: i8 = vec_to_int(&b.values) as i8;

        for i in 0..self.n {
            let bit = self.values[i].to_int();
            if bit == 1 {
                result = result ^ b;
            }
            b = b << 1;
            if b >= (1 << self.n) {
                b = b ^ vec_to_int(&self.primitive_poly) as i8;
            }
        }

        let mut result_vec = Vec::new();
        for _ in 0..self.n {
            result_vec.insert(0, GF2::new(result & 0b1));
            result = result >> 1;
        }

        FieldElement { n: self.n, values: result_vec, primitive_poly: self.primitive_poly.clone() }
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
