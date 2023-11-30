use std::fmt;
use rand::Rng;
use super::gf2::GF2;
use super::field_element::FieldElement;


// the entire field of dim n, i.e all binary vectors of length 2^n
pub struct Field {
    n: usize,
    values: Vec<FieldElement>,
    primitive_poly: Vec<GF2>,
}

impl Field {
    pub fn new(n: usize, primitive_poly: Vec<GF2>) -> Self {
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
            values.push(FieldElement::new(n, field_element, primitive_poly.clone()));
        }
        Field { n, values, primitive_poly }
    }
}

impl Field {
    pub fn from_integer(&self, num: u32) -> FieldElement {
        if num < 0 {
            panic!("FieldElement cannot be instantiated with a negative integer");
        } else if num >= 2u32.pow(self.n as u32) {
            panic!("FieldElement cannot be instantiated with an integer larger than 2^n");
        }

        let mut binary = format!("{:b}", num);
        while binary.len() < self.n {
            binary = format!("0{}", binary);
        }
        let mut field_element = Vec::new();
        for c in binary.chars() {
            field_element.push(GF2::new(c.to_digit(10).unwrap() as i8));
        }
        FieldElement::new(self.n, field_element, self.primitive_poly.clone())
    
    }
}

impl Field {
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn list(&self){
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