use std::ops::Sub;

use crate::Fix16;

impl Sub for &Fix16{
    type Output= Fix16;

    fn sub(self, rhs: Self) -> Self::Output {
        Fix16(self.0.saturating_sub(rhs.0))
    }
}
impl Sub<Fix16> for &Fix16{
    type Output= Fix16;

    fn sub(self, rhs: Fix16) -> Self::Output {
        self - &rhs
    }
}

impl Sub<&Fix16> for Fix16{
    type Output= Fix16;

    fn sub(self, rhs: &Fix16) -> Self::Output {
        &self - rhs 
    }
}
impl Sub for Fix16{
    type Output= Fix16;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}