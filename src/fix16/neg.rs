use std::ops::Neg;

use super::Fix16;

impl Neg for Fix16 {
    type Output = Fix16;

    fn neg(self) -> Self::Output {
        Fix16(-self.0)
    }
}