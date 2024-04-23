use std::fmt;

use super::Fix16;

impl fmt::Display for Fix16{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // This is just a quick hack but it works.
        (self.0 as f64 / Fix16::ONE.0 as f64).fmt(f)
    }
}