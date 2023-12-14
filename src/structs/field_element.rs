extern crate modulo;
use std::fmt;
use std::ops::{Add, Mul, Shl, AddAssign};
use std::cmp::PartialEq;
use super::gf2::{GF2};


#[derive(Clone)]
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


impl FieldElement {
    pub fn copy(&self) -> FieldElement {
        let mut new_values = Vec::new();
        for i in 0..self.n {
            new_values.push(self.values[i].clone());
        }
        FieldElement { n: self.n, values: new_values, primitive_poly: self.primitive_poly.clone() }
    }

}

impl <'a> Shl<u8> for &'a FieldElement {
    type Output = FieldElement; 
    fn shl(self, rhs: u8) -> FieldElement {
        let mut new_values = self.values.clone();
        // println!("primitive polynomial: {}", GF2Vec(self.primitive_poly.clone()));
        for _ in 0..rhs {
            if new_values[0].to_int() == 1 {
                new_values[0] = GF2::new(0);
                new_values.rotate_left(1);
                new_values = new_values.iter().zip(self.primitive_poly.iter()).map(|(a, b)| a + b).collect();
            } else {
                new_values.rotate_left(1);
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
            new_values.push(&self.values[i] + &other.values[i]);
        }
        FieldElement { n: self.n, values: new_values, primitive_poly: self.primitive_poly.clone() }
    }
}

impl AddAssign for FieldElement {
    fn add_assign(&mut self, other: Self) {
        for i in 0..self.n {
            self.values[i] += other.values[i];
        }
    }
}

impl<'a> Mul for &'a FieldElement {
    type Output = FieldElement; 


    fn mul(self, b: Self) -> FieldElement {

        // temporary variable for storing the shifted value
        let mut acc: FieldElement;

        let mut results: Vec<FieldElement> = Vec::new();
        let a_bits = self.values.clone().into_iter().rev().collect::<Vec<GF2>>();
        for i in 0..self.n {
             if a_bits[i].to_int() == 1 {
                 acc = b << i as u8;
                 results.push(acc.clone());
             }
        }

        let result: FieldElement = results.iter().fold(FieldElement::new(self.n, vec![GF2::new(0); self.n], self.primitive_poly.clone()), |acc, x| &acc + x);
        result
    }
    
}



impl FieldElement {
    pub fn pow(&self, n: u32) -> FieldElement {
        let mut result = self.clone();
        for _ in 0..n {
            result = &result * self;
        }
        result
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

impl FieldElement {
    pub fn to_int(&self) -> u32 {
        vec_to_int(&self.values)
    }
}

fn vec_to_int(values: &Vec<GF2>) -> u32 {
    let mut result = 0;
    for i in 0..values.len() {
        result += values[i].to_int() << i;
    }
    result
}

