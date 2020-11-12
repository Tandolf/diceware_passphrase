use rand::{
    thread_rng,
    Rng,
};
use rand::rngs::ThreadRng;

pub mod wordlist;
pub mod roller;

use crate::wordlist::{
    load,
    WordLists,
};
use crate::roller::Roller;

pub fn read_file() {

    let words = load(&WordLists::American);
    let mut roller = Roller::<ThreadRng>::new();
    let throws = roller.get_n_throws(6);

    let mut pass_phrase = String::new();
    for throw in throws {
        let t = throw.to_string();
        let word = words.get::<str>(&t).unwrap();
        pass_phrase += to_uppercase(word).as_str();
    }

    println!("{}", pass_phrase);
}

pub fn get_n_throws(throws: u32) -> String {
    // TODO: extract to own class init in Default
    let mut rand = thread_rng();
    let mut throw_str = String::new();
    for _ in 1..throws {
        let dice = rand.gen_range(1, 7);
        throw_str += dice.to_string().as_str();
    }
    throw_str
}

fn to_uppercase(value: &str) -> String {
    let mut chars: Vec<char> = value.chars().collect();
    chars[0] = chars[0].to_uppercase().nth(0).unwrap();
    let uc_word: String = chars.into_iter().collect();
    uc_word
}

#[cfg(test)]
mod tests {
    use crate::read_file;
    use crate::to_uppercase;

    #[test]
    fn it_works() {

        read_file();

        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn should_return_uppercase_word() {
        let word = "hello";
        assert_eq!("Hello", to_uppercase(word));
    }
}
