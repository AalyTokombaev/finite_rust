mod structs;

use structs::field::Field;
use structs::field_element::FieldElement;
use structs::gf2::GF2;

fn main() {
    /* 
        gonna use the primitive poly x^3 + x + 1 for now
        this is represented by the string 1011, we cut off the first 1 getting 011
    */
    let field = Field::new(4, vec![GF2::new(0), GF2::new(0), GF2::new(1), GF2::new(1)]);
    println!("{}", field);

    let x1: FieldElement = field.get_random_element();
    let x2: FieldElement = field.get_random_element();

    let y1 : FieldElement = field.from_integer(4);
    let y2 : FieldElement = field.from_integer(5);


    println!("x1: {}, x2: {}", x1, x2);

    let x3: FieldElement = &x1 + &x2;

    println!("x3: {}", x3);
    println!("x1: {}, x2: {}", x1, x2);

    let x4 = &x1 * &x2; /* remember mod i done */
    println!("x4 (x1*x2): {}", x4);


    let e1: GF2 = GF2::new(0);
    let e2: GF2 = GF2::new(1);

    println!("e1: {}, e2: {}", e1, e2);

    println!("Yeah!");
    

}
