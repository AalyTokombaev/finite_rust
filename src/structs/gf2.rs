use std::fmt;
use std::ops::Add;

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

// implement addition
impl Add for GF2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        GF2 { value: self.value ^ other.value }
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