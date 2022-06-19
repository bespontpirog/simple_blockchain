#[derive(Debug)]
pub struct FieldElement {
    num: u32,
    order: u32,
}

impl FieldElement {
    pub fn new(num: u32, order: u32) -> Self {
        assert!(num <= order, "Field order must be geather than num");

        FieldElement { num, order }
    }

    pub fn pow(self, power: u32) -> FieldElement {
        FieldElement::new(self.num.pow(power) % self.order, self.order)
    }
}

impl std::ops::Add<FieldElement> for FieldElement {
    type Output = FieldElement;

    fn add(self, rhs: FieldElement) -> FieldElement {
        assert!(self.num != rhs.num, "The order of fields must be equal");

        FieldElement::new((self.num + rhs.num) % self.order, self.order)
    }
}

impl std::ops::Mul<u32> for FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: u32) -> Self::Output {
        FieldElement::new((self.num * rhs) % self.order, self.order)
    }
}

impl std::ops::Mul<FieldElement> for u32 {
    type Output = FieldElement;
    fn mul(self, rhs: FieldElement) -> Self::Output {
        FieldElement::new((rhs.num * self) % rhs.order, rhs.order)
    }
}
