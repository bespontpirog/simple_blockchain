mod field;

use field::FieldElement;

fn main() {
    let f1 = FieldElement::new(6, 8);
    let f2 = FieldElement::new(2, 8);
    println!("{:?}", f1 + f2);
    let f1 = FieldElement::new(6, 8);
    let f2 = FieldElement::new(2, 8);
    println!("{:?}", f2 * f1);

    let f1 = FieldElement::new(6, 8);
    println!("{:?}", f1.pow(2));

    let f1 = FieldElement::new(6, 8);
    let f2 = FieldElement::new(2, 8);
    print!("{:?}", f1 / f2)
}
