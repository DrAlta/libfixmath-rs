use super::Fix16;

impl Fix16 {
    pub fn atan2(&self, x: &Self) -> Self {
        let in_y = self;
        let in_x = x;
        let abs_in_y: Fix16 = self.abs();
        let mut angle: Fix16;
        let r: Fix16;
        let r_3: Fix16;
    
    
        if  in_x.0 >= 0 {
            r = (in_x - &abs_in_y) / (in_x + &abs_in_y);
            r_3 = (&r * &r) * &r;
            angle = (Fix16(0x00003240) * r_3) - (Fix16(0x0000FB50) * &r) + Fix16::PI_DIV_4;
        } else {
            r =  (in_x + &abs_in_y) / (&abs_in_y - in_x);
            r_3 = (&r * &r)  * &r;
            angle = (Fix16(0x00003240) * r_3)
                - (Fix16(0x0000FB50) * &r)
                + Fix16::THREE_PI_DIV_4;
        }
        if in_y.0 < 0 {
            angle = -angle;
        }
    
        return angle;
    
    }
}

/*
#[cfg(test)]
mod tests {
    use super::Fix16;
    #[test]
    fn atan2_one_two() {
        let fix16 = format!("{:0.2}",Fix16::ONE.atan2(&Fix16::TWO));
        let f64 = format!("{:0.2}",1_f64.atan2(2.0));
        assert_eq!(fix16, f64);
    }
}
*/