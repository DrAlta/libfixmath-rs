use super::Fix16;
impl Fix16 {
    pub fn abs(&self) -> Self {
        Fix16(self.0.abs())
        /*
        if self.0.is_negative(){
            Fix16(-self.0)
        } else {
            self.clone()
        }*/
    }
}


#[cfg(test)]
mod tests {
    use super::Fix16;
    #[test]
    fn abs_neg_one() {
        assert_eq!(Fix16::NEG_ONE.abs(), Fix16::ONE);
    }
}