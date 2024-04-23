use std::ops::Rem;

use super::Fix16;

impl Rem for &Fix16{
    type Output = Fix16;

    fn rem(self, rhs: Self) -> Self::Output {
        Fix16(
        self.0 % rhs.0
        )
    }
}


impl Rem<Fix16> for &Fix16{
    type Output= Fix16;

    fn rem(self, rhs: Fix16) -> Self::Output {
        self % &rhs
    }
}


impl Rem<&Fix16> for Fix16{
    type Output= Fix16;

    fn rem(self, rhs: &Fix16) -> Self::Output {
        &self % rhs
    }
}


impl Rem for Fix16{
    type Output= Fix16;

    fn rem(self, rhs: Self) -> Self::Output {
        &self % &rhs
    }
}


#[cfg(test)]
mod tests {
    use super::Fix16;
    #[test]
    fn three_rem_two() {
        assert_eq!(&Fix16::THREE % &Fix16::TWO, Fix16::ONE);
    }
}