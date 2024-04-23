pub fn i128_to_i32(value:i128) -> i32 {
    if value > i32::MAX as i128{
        i32::MAX
    } else {
        value as i32
    }
}


pub fn i64_to_i32(value:i64) -> i32 {
    if value > i32::MAX as i64 {
        i32::MAX
    } else {
        value as i32
    }
}