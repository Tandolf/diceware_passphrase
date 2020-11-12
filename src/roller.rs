use std::vec::Vec;
use rand::{thread_rng, Rng};
use rand::rngs::ThreadRng;
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub struct Roller<R: Rng> {
    rand: R,
}

impl<R: Rng> Roller<R> {
    pub fn new() -> Roller<ThreadRng> {
        let rand = thread_rng();
        Roller::<ThreadRng> { rand }
    }

    pub fn from(rand: R) -> Roller<R> {
        Roller { rand }
    }

    pub fn get_u32(&mut self) -> u32 {
        *&self.rand.next_u32()
    }

    pub fn get_n_throws(&mut self, number: u32) -> Vec<Throw> {
        let mut throws = Vec::new();
        for _ in 1..number + 1 {
            let rolls = self.get_n_rolls(5);
            let throw = Throw::new(rolls);
            throws.push(throw);
        }   
        throws
    }

    fn get_n_rolls(&mut self, n: u32) -> u32 {
        let mut sum = 0;
        for _ in 1..n + 1 {
            let roll = self.rand.gen_range(1, 7);
            sum = (sum * 10) + roll;
        }
        sum
    }
}

#[cfg(test)]
mod test {

    use crate::Roller;
    use rand::rngs::mock::StepRng;

    #[test]
    fn should_return_random_number() {
        let mut roller = Roller::from(StepRng::new(5, 1));
        assert_eq!(roller.get_u32(), 5);
    }

    #[test]
    fn should_return_n_throws_as_number() {
        let mut roller = Roller::from(StepRng::new(1, 1));
        assert_eq!(roller.get_n_rolls(3), 111);
    }

    #[test]
    fn should_return_n_number_of_throws() {
        let mut roller = Roller::from(StepRng::new(1, 1));
        assert_eq!(roller.get_n_throws(3).len(), 3)
    }
}

#[derive(Debug, Clone)]
pub struct Throw {
    throw: u32,
}

impl Throw {
    pub fn new(throw: u32) -> Throw {
        Throw { throw }
    }
}

impl Display for Throw {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.throw.to_string())
    }
}