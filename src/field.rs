#[derive(PartialEq, Debug)]
pub struct FieldElement {
    num: u32,
    order: u32,
}

impl FieldElement {
    pub fn new(num: u32, order: u32) -> Self {
        if num >= order {
            panic!("Field order must be geather than num");
        }

        FieldElement { num, order }
    }

    pub fn pow(self, power: u32) -> FieldElement {
        FieldElement::new(self.num.pow(power) % self.order, self.order)
    }
}

impl std::ops::Add<FieldElement> for FieldElement {
    type Output = FieldElement;

    fn add(self, rhs: FieldElement) -> FieldElement {
        if self.order != rhs.order {
            panic!("The order of fields must be equal");
        }

        FieldElement::new((self.num + rhs.num) % self.order, self.order)
    }
}

impl std::ops::Mul<FieldElement> for FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: FieldElement) -> Self::Output {
        if self.order != rhs.order {
            panic!("The order of fields must be equal");
        }

        FieldElement::new((rhs.num * self.num) % rhs.order, rhs.order)
    }
}

impl std::ops::Div<FieldElement> for FieldElement {
    type Output = FieldElement;

    fn div(self, rhs: FieldElement) -> Self::Output {
        if self.order != rhs.order {
            panic!("The order of fields must be equal");
        }

        let back_rhs = rhs.pow(self.order - 2);
        self * back_rhs
    }
}

#[cfg(test)]
mod tests {
    use super::FieldElement;

    #[test]
    #[should_panic(expected = "Field order must be geather than num")]
    fn test_new_panic() {
        _ = FieldElement::new(10, 1);
    }

    #[test]
    fn test_add() {
        assert_eq!(
            FieldElement::new(5, 8),
            FieldElement::new(3, 8) + FieldElement::new(2, 8)
        );
        assert_eq!(
            FieldElement::new(4, 8),
            FieldElement::new(5, 8) + FieldElement::new(7, 8)
        );
    }

    #[test]
    #[should_panic(expected = "The order of fields must be equal")]
    fn test_add_panic() {
        _ = FieldElement::new(1, 2) + FieldElement::new(2, 3)
    }

    #[test]
    fn test_mul() {
        assert_eq!(
            FieldElement::new(6, 8),
            FieldElement::new(3, 8) * FieldElement::new(2, 8)
        );
        assert_eq!(
            FieldElement::new(4, 8),
            FieldElement::new(3, 8) * FieldElement::new(4, 8)
        );
    }

    #[test]
    #[should_panic(expected = "The order of fields must be equal")]
    fn test_mul_panic() {
        _ = FieldElement::new(1, 2) * FieldElement::new(2, 3)
    }

    #[test]
    fn test_pow() {
        assert_eq!(FieldElement::new(3, 8), FieldElement::new(3, 8).pow(3))
    }

    #[test]
    fn test_div() {
        assert_eq!(
            FieldElement::new(2, 4),
            FieldElement::new(2, 4) / FieldElement::new(3, 4)
        );
    }

    #[test]
    #[should_panic(expected = "The order of fields must be equal")]
    fn test_div_panic() {
        _ = FieldElement::new(1, 2) / FieldElement::new(2, 3)
    }
}
