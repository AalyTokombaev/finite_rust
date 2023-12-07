use std::fmt;
use std::ops::{Add, Mul};

/*
So let's think for a bit here... I think working with vectors is going to be a lot easier than working with
actual field elements, so our FieldElement can just be a boolean.
*/

// let's call it gf2 for now..
#[derive(Copy, Clone)]
pub struct GF2 {
    value: bool // we can use 0 or 1 in the instansiation :D 
}

impl GF2 {
    pub fn new(value: i8) -> Self {
        if value == 0 {
            GF2 { value: false }
        } else if  value == 1 {
            GF2 { value: true }
        }
        else {
            panic!("GF2 can only be instantiated with 0 or 1");
        }
    }
}

impl GF2 {
    pub fn clone(&self) -> Self {
        GF2 { value: self.value }
    }

}

impl GF2 {
    pub fn to_int(&self) -> u32 {
        if self.value {
            return 1;
        }
        else {
            return 0;
        }
    }
}

impl GF2 {
    pub fn to_string (&self) -> String {
        match self.value {
            true => String::from("1"),
            false => String::from("0")
        }
    }
}

// implement addition
impl <'a> Add for &'a GF2 {
    type Output = GF2;

    fn add(self, other: Self) -> GF2 {
        GF2 { value: self.value ^ other.value }
    }
}

impl Mul for GF2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        GF2 { value: self.value & other.value }
    }
}


impl fmt::Display for GF2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.value {
            return write!(f, "1");
        }
        else {
            return write!(f, "0");
        }
    }
}


impl PartialEq for GF2 {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

pub struct GF2Vec(pub Vec<GF2>);

#[allow(dead_code)]
impl fmt::Display for GF2Vec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        s.push('[');
        for i in 0..self.0.len() {
            s.push_str(&format!(" {} ", self.0[i]));
        }
        s.push(']');
        write!(f, "{}", s)
    }
}
