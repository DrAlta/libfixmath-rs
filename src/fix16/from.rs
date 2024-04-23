use crate::util::i128_to_i32;
use super::Fix16;

impl From<i16> for Fix16 {
    fn from(item: i16) -> Self {
        Self(item as i32 * Self::ONE.0)
    }
}


impl From<(i64, u64)> for Fix16 {
    fn from(value: (i64, u64)) -> Self {
        let num = value.0 as i128 * 65536;
        let body = num / value.1 as i128;
        Fix16(i128_to_i32(body))
    }
}