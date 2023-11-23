use std::fmt;
use rand::Rng;
use super::gf2::GF2;
use super::field_element::FieldElement;


// the entire field of dim n, i.e all binary vectors of length 2^n
pub struct Field {
    n: usize,
    values: Vec<FieldElement>
}

impl Field {
    pub fn new(n: usize) -> Self {
        let mut values = Vec::new();
        for i in 0..(2usize.pow(n as u32)) {
            let mut binary = format!("{:b}", i);
            while binary.len() < n {
                binary = format!("0{}", binary);
            }
            let mut field_element = Vec::new();
            for c in binary.chars() {
                field_element.push(GF2::new(c.to_digit(10).unwrap() as i8));
            }
            values.push(FieldElement::new(n, field_element));
        }
        Field { n, values }
    }
}

impl Field {
    pub fn contains(&self, field_element: &FieldElement) -> bool {
        for i in 0..self.values.len() {
            if self.values[i] == *field_element {
                return true;
            }
        }
        false
    }
}

impl Field {
    // prints a list of the field elements
    fn list(&self){
        for i in 0..self.values.len() {
            println!("{}", self.values[i]);
        }
    }
}

impl Field {
    pub fn get_random_element(&self) -> FieldElement {
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..self.values.len());

        self.values[i].clone()
    
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write! (f, "Finite Field of size 2^{}:\n", self.n);
    }
}