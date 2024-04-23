use super::Fix16;


impl Fix16 {
    pub fn atan(&self) -> Self {
        self.atan2(&Self::ONE)
    }
}
/*
#[cfg(test)]
mod tests {
    use super::Fix16;
    #[test]
    fn atan_half() {
        let fix16 = format!("{:0.2}",Fix16::HALF.atan());
        let f64 = format!("{:0.2}",0.5_f64.atan());
        assert_eq!(fix16, f64);
    }
}
*/