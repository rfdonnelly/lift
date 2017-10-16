use std::fmt;

pub struct Set {
    pub weight: u32,
    pub reps: u32,
    pub sets: u32,
}

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}x{}",
               self.weight, self.reps, self.sets)
    }
}

impl fmt::Debug for Set {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}x{}",
               self.weight, self.reps, self.sets)
    }
}
