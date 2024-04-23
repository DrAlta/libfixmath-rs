use std::ops::Add;

use super::Fix16;

impl Add for &Fix16{
    type Output= Fix16;

    fn add(self, rhs: Self) -> Self::Output {
        Fix16(self.0.saturating_add(rhs.0))
    }
}


impl Add<Fix16> for &Fix16{
    type Output= Fix16;

    fn add(self, rhs: Fix16) -> Self::Output {
        self + &rhs
    }
}


impl Add<&Fix16> for Fix16{
    type Output= Fix16;

    fn add(self, rhs: &Fix16) -> Self::Output {
        &self + rhs
    }
}


impl Add for Fix16{
    type Output= Fix16;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}


#[cfg(test)]
mod tests {
    use super::Fix16;
    #[test]
    fn one_add_two() {
        assert_eq!(&Fix16::ONE + &Fix16::TWO, Fix16::THREE);
    }
}