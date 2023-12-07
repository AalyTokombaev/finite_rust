extern crate modulo;
use std::fmt;
use std::ops::{Add, Mul, Shl};
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

impl Shl<u8> for FieldElement {
    type Output = FieldElement;
    fn shl(self, rhs: u8) -> FieldElement {
        let mut modulate = false;
        if self.values[0].to_int() == 1 {
            modulate  = true;
        }
        let mut new_values = self.values.clone();
        for _ in 0..rhs {
            new_values.rotate_left(1);
        }
        if modulate {
            // prefrom a xor with the primitive poly
            for i in 0..self.n {
                new_values[i] = new_values[i] + self.primitive_poly[i];
            }
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
        let mut b_tmp = b.clone();
        let mut R = FieldElement::new(self.n, vec![GF2::new(0); self.n], self.primitive_poly.clone());
        for i in 0..self.n {
           b_tmp=  b_tmp << (1 as u8) ;
           println!("i : {}, b_tmp: {}", i, b_tmp);
           if self.values[i].to_int() == 1 {
               R = &R + &b_tmp;
           }
        }
        R
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
