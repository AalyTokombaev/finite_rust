
use super::field::Field;
use super::field_element::FieldElement;
use super::gf2::GF2;
use std::fmt::Display;

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


impl LinearFunction {
    pub fn evaluat(&self, x: FieldElement) -> FieldElement {
        // linear functions are  of the form
        // a^x + a^x^2 + a^x^4 + a^x^8 + ... + a^x^2^n
        // where n is the size of the field,
        let n = self.coefficients[0].n; // the size of the field
        let mut result: FieldElement = self.field.from_integer(0);
        for i in 0..self.coefficients.len() {

        }:



    }
}


     fmt::Display for LinearFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut result = String::new();
        for i in 0..self.coefficients.len() {
            result.push_str(&format!("{}x^{} + ", self.coefficients[i], i.pow(2.pow(i))));
        }

        write!(f, "{}", result)
    }
}


