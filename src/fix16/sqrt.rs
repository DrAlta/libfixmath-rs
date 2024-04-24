use num_integer::Roots;

use super::Fix16;

impl Fix16 {
    pub fn sqrt(&self) -> Self {
         let denom = 65536_u128;
         let numer = self.0.unsigned_abs() as u128;
         
         let n = (denom * numer).sqrt();
         
         Self(n as i32)
     }
     /*
    pub fn sqrt(&self) -> Self {
        let neg: u8 = if self.0 < 0 {0xFF} else {0x00};
        let mut num    = self.0.unsigned_abs();
        let mut result: u32 = 0;
        let mut bit: u32;
        let mut n: u8;
    
        // Many numbers will be less than 15, so
        // this gives a good balance between time spent
        // in if vs. time spent in the while loop
        // when searching for the starting value.
        if num & 0xFFF00000 != 0 {
            bit = 1 << 30;
        } else {
            bit = 1 << 18;
        }
    
        while bit > num {
            bit >>= 2;
        }
        // The main part is executed twice, in order to avoid
        // using 64 bit values in computations.
        for idx in 0..2 { //for (n = 0; n < 2; n++)
            n = idx;
            // First we get the top 24 bits of the answer.
            while bit != 0 {
                if num >= result + bit {
                    num -= result + bit;
                    result = (result >> 1) + bit;
                } else {
                    result = result >> 1;
                }
                bit >>= 2;
            }
    
            if n == 0 {
                // Then process it again to get the lowest 8 bits.
                if num > 65535 {
                    // The remainder 'num' is too large to be shifted left
                    // by 16, so we have to add 1 to result manually and
                    // adjust 'num' accordingly.
                    // num = a - (result + 0.5)^2
                    //	 = num + result^2 - (result + 0.5)^2
                    //	 = num - result - 0.5
                    num -= result;
                    num    = (num << 16) - 0x8000;
                    result = (result << 16) + 0x8000;
                } else {
                    num <<= 16;
                    result <<= 16;
                }
    
                bit = 1 << 14;
            }
        }
    
        let result: i32 = result.try_into().unwrap();
        if neg != 0  {Fix16(-result) } else { Fix16(result)}
    }
*/
}


#[cfg(test)]
mod tests {
    use super::Fix16;
    #[test]
    fn sqrt_four() {
        assert_eq!(Fix16::FOUR.sqrt(), Fix16::TWO);
    }
}