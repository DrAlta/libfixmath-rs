use std::ops::Div;

use super::Fix16;
use crate::util::i64_to_i32;

impl Div for &Fix16{
    type Output = Fix16;

    fn div(self, rhs: Self) -> Self::Output {
        let lhs = self.0 as i64 * (65536);
        let rhs = rhs.0 as i64 ;//* (65536/2);
        let x = lhs.saturating_div(rhs);
        Fix16(i64_to_i32(x))
        
    }
}


impl Div<Fix16> for &Fix16{
    type Output= Fix16;

    fn div(self, rhs: Fix16) -> Self::Output {
        self / &rhs
    }
}


impl Div<&Fix16> for Fix16{
    type Output= Fix16;

    fn div(self, rhs: &Fix16) -> Self::Output {
        &self / rhs
    }
}


impl Div for Fix16{
    type Output= Fix16;

    fn div(self, rhs: Self) -> Self::Output {
        &self / &rhs
    }
}


#[cfg(test)]
mod tests {
    use super::Fix16;

    #[test]
    fn one_div_two() {
        assert_eq!(Fix16::ONE / Fix16::TWO, Fix16::HALF);
    }


    #[test]
    fn two_div_two() {
        assert_eq!(&Fix16::TWO / &Fix16::TWO, Fix16::ONE);
    }
}