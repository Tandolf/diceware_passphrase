use std::fmt::Display;
#[derive(Debug, Clone)]
struct Phrase {
    words: Vec<String>,
    index: usize
}

impl Phrase {

    fn new(words: Vec<String>) -> Self {
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
        let s = self.words[self.index].clone();
        if self.index == self.words.len() {
            None
        } else {
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
}