use std::vec::Vec;
use rand::{thread_rng, Rng};
use rand::rngs::ThreadRng;
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub struct Roller {
    rand: ThreadRng,
}

impl Roller {
    pub fn new() -> Roller {
        let rand = thread_rng();
        Roller { rand }
    }

    pub fn get_n_throws(&mut self, number: u32) -> Throws {
        let mut throws = Vec::new();
        for _ in 1..number {

            let mut counter = 0;
            let mut current_sum = self.rand.gen_range(1, 7);
            let dice = loop {
                
                if counter == 4 {
                    break current_sum;
                }

                let roll = self.rand.gen_range(1, 7);
                current_sum = (current_sum * 10) + roll;
                counter += 1;
            };

            throws.push(dice);
        }   
        Throws { throws }
    }
}

#[derive(Debug, Clone)]
pub struct Throws {
    throws: Vec<u32>,
}

impl Throws {
    pub fn get(&self, value: usize) -> u32 {
        let value = self.throws[value];
        value
    }

    pub fn get_as_string(&self, value: usize) -> String {
        let value = self.throws[value];
        let value = value.to_string();
        value
    }
}

impl Display for Throws {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut throws_string = String::new();
        for throw in &self.throws {
            throws_string += throw.to_string().as_str();
        }
        write!(f, "{}", throws_string)
    }
}