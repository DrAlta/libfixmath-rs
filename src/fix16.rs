mod abs;
mod add;
#[cfg(feature = "trig")]
mod atan;
#[cfg(feature = "trig")]
mod atan2;
mod constants;
mod display;
mod div;
mod from;
mod mul;
mod neg;
mod rem;
mod sqrt;
mod sub;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fix16(i32);

impl Fix16{
    pub fn new(inner: i32) -> Self{
        Self(inner)
    }
    pub fn inner(&self) -> &i32 {
        &self.0
    }
}