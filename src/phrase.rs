use std::fmt::Display;
#[derive(Debug, Clone)]
pub struct Phrase {
    words: Vec<String>,
    index: usize
}

impl Phrase {

    pub fn new(words: Vec<String>) -> Self {
        Phrase { words: words, index: 0 }
    }

    fn set_special_char(&mut self, index: usize, letter: usize, new_char: char) {
        let word = &self.words[index];

        let mut new_word = String::new();
        for (i, mut char) in word.chars().enumerate() {
            if i == letter { char = new_char}
            new_word.push(char)
        }
        self.words[index] = new_word;
    }

    pub fn to_capitalized(&mut self) -> Self {
        let words = self.words.iter_mut().map(|w| Phrase::to_uppercase(&w)).collect();
        Phrase { words: words, index: 0}
    }

    fn to_uppercase(value: &str) -> String {
        let mut chars: Vec<char> = value.chars().collect();
        chars[0] = chars[0].to_uppercase().nth(0).unwrap();
        chars.into_iter().collect()
    }
}

impl Display for Phrase {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let phrase = self.words.concat();
        write!(f, "{}", phrase)
    }
}

impl Iterator for Phrase {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.words.len() == 0 {
            return None
        }

        if self.index == self.words.len() {
            None
        } else {
            let s = self.words[self.index].clone();
            self.index += 1;
            Some(s)
        }
    }
}

#[cfg(test)]
mod test {

    use crate::phrase::Phrase;

    #[test]
    fn should_display_phrase_correct_if_printed() {
        let words = vec!(String::from("Hello"), String::from("World"));
        let phrase = Phrase::new(words);
        assert_eq!("HelloWorld", phrase.to_string());
    }

    #[test]
    fn should_replace_single_char() {
        let words = vec!(String::from("Hello"), String::from("World"));
        let mut phrase = Phrase::new(words);
        phrase.set_special_char(0, 0, '!');
        assert_eq!("!elloWorld", phrase.to_string());
    }

    #[test]
    fn should_assert_iterator() {
        let words = vec!(String::from("Hello"), String::from("World"));
        let mut phrase = Phrase::new(words);
        assert_eq!(phrase.next(), Some(String::from("Hello")));
        assert_eq!(phrase.next(), Some(String::from("World")));
    }

    #[test]
    fn should_test_empty_vec() {
        let v = vec!();
        let mut phrase = Phrase::new(v);
        assert_eq!(phrase.next(), None);
    }

    #[test]
    fn should_test_iteration() {
        let v = vec!(String::from("Hello"));
        let phrase = Phrase::new(v);

        for word in phrase {
            assert_eq!(word.to_string(), word);
        }
    }

    #[test]
    fn should_test_uppercase_single_word() {
        assert_eq!(Phrase::to_uppercase("hello"), "Hello");
    }

    #[test]
    fn should_capitalize() {
        let words = vec!(String::from("hello"), String::from("world"));
        let mut phrase = Phrase::new(words);
        let phrase = phrase.to_capitalized();
        assert_eq!(phrase.to_string(), "HelloWorld");
    }
}