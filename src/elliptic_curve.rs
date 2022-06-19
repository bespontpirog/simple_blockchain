#[derive(PartialEq, Debug)]
pub struct Point {
    x: i32,
    y: i32,
    a: i32,
    b: i32,
}

impl Point {
    /// Point constructor.
    ///
    /// Form of the curve:
    /// ```
    /// y^2 = x^3 + ax + b
    /// ```
    pub fn new(x: i32, y: i32, a: i32, b: i32) -> Self {
        assert!(
            y.pow(2) == x.pow(3) + a * x + b,
            "{}",
            format!("Point {} {} {} {} is not a point of curve", x, y, a, b)
        );
        Point {
            x: x,
            y: y,
            a: a,
            b: b,
        }
    }
}
