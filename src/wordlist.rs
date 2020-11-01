use std::include_str;
use std::collections::HashMap;

pub enum WordLists {
    Swedish,
    American
}

pub fn load(word_list: &WordLists) -> HashMap<&str, &str>{

    let file = match word_list {
        WordLists::Swedish => include_str!("../resources/swedish_list.txt"),
        WordLists::American => include_str!("../resources/beale_list.txt"),
    };

    let split_words: Vec<&str> = file.split("\n").collect();
    let mut words = HashMap::new();
    for line in split_words.iter() {
    let key_value: Vec<&str> = line.split_whitespace().collect();
        words.insert(key_value[0], key_value[1]);
    }

    words
}