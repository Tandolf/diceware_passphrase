use rand::rngs::ThreadRng;

pub mod wordlist;
pub mod roller;
pub mod phrase;

use crate::wordlist::{
    load,
    WordLists,
};
use crate::roller::Roller;
use crate::phrase::Phrase;

pub fn read_file() {

    let dictionary = load(&WordLists::American);
    let mut roller = Roller::<ThreadRng>::new();
    let throws = roller.get_n_throws(6);

    let words: Vec<String> = throws.iter().map(|t| dictionary[t.to_string().as_str()]
                                                            .to_owned()
                                                            .clone())
                                                        .collect();
    let mut phrase = Phrase::new(words);

    println!("{}", phrase.to_capitalized());
}



#[cfg(test)]
mod tests {
    use crate::read_file;

    #[test]
    fn it_works() {

        read_file();

        assert_eq!(2 + 2, 4);
    }
}
