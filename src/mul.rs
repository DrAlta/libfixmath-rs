use std::ops::Mul;

use crate::{constants::fix16_overflow, Fix16};
impl Mul for &Fix16{
    type Output = Fix16;

    fn mul(self, rhs: Self) -> Self::Output {
        let product = self.0 as i64 * rhs.0 as i64;

        let upper = product >> 47;
        if product < 0 {
		    if upper.reverse_bits() != 0 {
				return fix16_overflow;
            }
        } else {
            if upper != 0 {
				return fix16_overflow;
            }
        }

        Fix16(((product >> 16) % i32::MAX as i64) as i32)
	
    }
}