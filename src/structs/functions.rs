
use super::field::Field;
use super::field_element::FieldElement;
#[allow(unused_imports)]
use super::gf2::GF2;
use std::fmt;

pub struct LinearFunction {
    // let's assume that 
    field: Field,
    coefficients: Vec<FieldElement>,

}


impl LinearFunction {
    pub fn new(field: Field, coefficients: Vec<FieldElement>) -> LinearFunction {
        LinearFunction {
            field,
            coefficients,
        }
    }
}


impl LinearFunction {
    pub fn evaluat(&self, x: FieldElement) -> FieldElement {
        // linear functions are  of the form
        // a*x + a*x^2 + a*x^4 + a*x^8 + ... + a*x^(2^n)
        // where n is the size of the field,
        let result = self.field.from_integer(0);


        result
    }
}


impl fmt::Display for LinearFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut result = String::new();
        for i in 0..self.coefficients.len() {
            result.push_str(&format!("{}x^{} + ", self.coefficients[i], i.pow(2u32.pow(i as u32))));
        }

        write!(f, "{}", result)
    }
}

pub struct QuadraticFunction {
    field: Field,
    coefficients: Vec<FieldElement>,
}

impl QuadraticFunction {
    pub fn new(field: Field, coefficients: Vec<FieldElement>) -> QuadraticFunction {
        QuadraticFunction {
            field,
            coefficients,
        }
    }
}


impl QuadraticFunction {
    pub fn evaluate(&self, x: FieldElement) -> FieldElement {
        // quadratic functions are of the form
        // for i, j with 0 <= i < j < n
        //  we get a*x^(2^i) * x^(2^j)
        // where n is the size of the field,
        let mut result = self.field.from_integer(0);
        for i in 0.self.coefficients.len() {
            for j in i+1..self.coefficients.len() {
                result += self.coefficients[i] * self.coefficients[j] * x.pow(2u32.pow(i as u32) + 2u32.pow(j as u32));
            }
        }

        result
    }
}






