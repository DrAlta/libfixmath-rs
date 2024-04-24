use crate::Fix16;

impl Fix16{
    pub fn powi(&self, p: i32) -> Self {
        if p == 0 {
            return Fix16::ONE;
        }

        let n = (self.0.unsigned_abs() as u128).checked_pow(p.unsigned_abs()).expect("the numer over flowed");
        let d = 65536_u128.pow(p.unsigned_abs());

        let (numer, denom) = if p < 0 {
            ( d, n, )
        } else {
            ( n, d,)
        };
        println!(
            "{} to the {p} is {}",
            self.0 as f64 / 65536.0 ,
             n as f64 / d as f64);
        let x= fraction_to_fix16( numer, denom);
        println!("final in {x}");
        x
    }
}

fn fraction_to_fix16(numer: u128, denom: u128) -> Fix16 {
    Fix16::new(((numer * 65536) / denom) as i32)
}

#[cfg(test)]
mod tests {

    use crate::Fix16;
    #[test]
    fn one_half_to_the_two() {
        let value = Fix16::HALF.powi(2); 
        assert_eq!(
            value,
            Fix16::FOURTH
        );
    }
    #[test]
    fn neg_one_to_the_two() {
        let value = Fix16::NEG_ONE.powi(2); 
        assert_eq!(
            value,
            Fix16::ONE
        );
    }
    #[test]
    fn two_to_the_neg_two() {
        let value = Fix16::TWO.powi(-2); 
        assert_eq!(
            value,
            Fix16::FOURTH
        );
    }
}