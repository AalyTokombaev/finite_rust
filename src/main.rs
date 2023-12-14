mod structs;

use structs::field::Field;
use structs::field_element::FieldElement;
use structs::gf2::GF2;
use structs::functions::{LinearFunction, QuadraticFunction};



fn test_function(field: &Field, x: &FieldElement) -> FieldElement {
    // gonna test it for GF(2^3) for now
    let a_01: FieldElement = field.from_integer(1); // 001
    let a_02: FieldElement = field.from_integer(4); // 100
    let a_12: FieldElement = field.from_integer(2); // 010
    let elements = vec![a_01.clone(), a_02.clone(), a_12.clone()];
    // recall that F = a_01 * x^{2^0 + 2^1} + a_02* + a_12
    &(&(&a_01 * &x.pow(3)) + &(&a_02 * &x.pow(4))) + &(&a_12 * &x.pow(6))
    
}

fn main() {
    /* 
        gonna use the primitive poly x^3 + x + 1 for now
        this is represented by the string 1011, we cut off the first 1 getting 011
    */
    let primitive_poly = vec![GF2::new(0), GF2::new(1), GF2::new(1)];
    let field = Field::new(3, primitive_poly);
    println!("{}", field);

    let x1: FieldElement = field.from_integer(5);
    let x2: FieldElement = field.from_integer(3);

    let _y1 : FieldElement = field.from_integer(4);
    let _y2 : FieldElement = field.from_integer(5);


    println!("x1: {}, x2: {}", x1, x2);

    let x4 = &x1 * &x2; 
    println!("x4 (x1*x2): {}", x4);

    let c1: FieldElement = field.from_integer(1);
    let c2: FieldElement = field.from_integer(2);
    let c3: FieldElement = field.from_integer(3);
    let c4: FieldElement = field.from_integer(4);


    let f: QuadraticFunction = QuadraticFunction::new(field, vec![c1, c2, c3]);

}
