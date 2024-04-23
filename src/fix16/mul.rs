use std::ops::Mul;

use crate::util::i64_to_i32;
use super::Fix16;

impl Mul for &Fix16{
    type Output = Fix16;

    fn mul(self, rhs: Self) -> Self::Output {
        let product = self.0 as i64 * rhs.0 as i64;

        let upper = product >> 47;
        if product < 0 {
		    if upper.reverse_bits() != 0 {
				return Fix16::OVERFLOW;
            }
        } else {
            if upper != 0 {
				return Fix16::OVERFLOW;
            }
        }

        Fix16(i64_to_i32(product >> 16))
	
    }
}


impl Mul<Fix16> for &Fix16{
    type Output= Fix16;

    fn mul(self, rhs: Fix16) -> Self::Output {
        self * &rhs
    }
}


impl Mul<&Fix16> for Fix16{
    type Output= Fix16;

    fn mul(self, rhs: &Fix16) -> Self::Output {
        &self * rhs
    }
}


impl Mul for Fix16{
    type Output= Fix16;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}


#[cfg(test)]
mod tests {
    use super::Fix16;
    #[test]
    fn two_mul_two() {
        assert_eq!(&Fix16::TWO * &Fix16::TWO, Fix16::FOUR   );
    }
}