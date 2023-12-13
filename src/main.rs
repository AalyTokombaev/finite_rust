mod structs;

use structs::field::Field;
use structs::field_element::FieldElement;
use structs::gf2::GF2;



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
    let primitive_poly = vec![GF2::new(0), GF2::new(0), GF2::new(1), GF2::new(1)];
    let field = Field::new(4, primitive_poly);
    println!("{}", field);

    let x1: FieldElement = field.from_integer(5);
    let x2: FieldElement = field.from_integer(3);

    let _y1 : FieldElement = field.from_integer(4);
    let _y2 : FieldElement = field.from_integer(5);


    println!("x1: {}, x2: {}", x1, x2);

    let x4 = &x1 * &x2; 
    println!("x4 (x1*x2): {}", x4);


    let e1: GF2 = GF2::new(0);
    let e2: GF2 = GF2::new(1);

    println!("e1: {}, e2: {}", e1, e2);


    println!("Testing function");
    for i in 0..16{
        let x = field.from_integer(i);
        println!("x: {}, f(x): {}", x, test_function(&field, &x));
    }
    
}
