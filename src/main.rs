mod structs;

use structs::field::Field;
use structs::field_element::FieldElement;
use structs::gf2::GF2;

fn main() {
    /* 
        gonna use the primitive poly x^3 + x + 1 for now
        this is represented by the string 1011, we cut off the first 1 getting 011
    */
    let primitive_poly = vec![GF2::new(0), GF2::new(0), GF2::new(1), GF2::new(1)];
    let field = Field::new(4, primitive_poly);
    println!("{}", field);

    /*
    let x1: FieldElement = field.get_random_element();
    let x2: FieldElement = field.get_random_element();
    */ 
    let x1: FieldElement = field.from_integer(5);
    let x2: FieldElement = field.from_integer(3);

    let _y1 : FieldElement = field.from_integer(4);
    let _y2 : FieldElement = field.from_integer(5);


    




    let x3: FieldElement = &x1 + &x2;

    println!("x1: {}, x2: {}", x1, x2);

    let x4 = &x1 * &x2; /* remember mod i done */
    println!("x4 (x1*x2): {}", x4);


    let e1: GF2 = GF2::new(0);
    let e2: GF2 = GF2::new(1);

    println!("e1: {}, e2: {}", e1, e2);

    println!("Yeah!");

    let b = field.from_integer(3);
    println!("b: {}", b);
    println!("b << 1 {}", &b << 1);
    println!("b << 2 {}", &b << 2);
    println!("b << 3 {}", &b << 3);
    println!("b << 4 {}", &b << 4);
    println!("b << 5 {}", &b << 5);

    

}
