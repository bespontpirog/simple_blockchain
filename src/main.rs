mod elliptic_curve;
mod field;

use crate::elliptic_curve::Point;
use crate::field::FieldElement;

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
    println!("{:?}", f1 / f2);

    let p1 = Point::new(-1, -1, 5, 7);
    let p2 = Point::new(-1, -1, 5, 7);
    println!("{:?}", p1 != p2);
}
