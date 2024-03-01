fn find_first_word(sentence: &str) -> &str {
    let first_word = sentence.split_whitespace().next();
    match first_word {
        Some(word) => word,
        None => sentence
    }
}

fn find_first_word_oldfashion(sentence: &str) -> &str {
    let mut i: usize = 0;
    for c in sentence.chars(){
        if c == ' ' {
            return &sentence[0..i];
        }
        i += 1;
    }
    return sentence;
}
fn main() {
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex5_1_can_find_first_word() {
        assert_eq!("Hello", find_first_word("Hello World"));
        assert_eq!("Hello", find_first_word("Hello"));
        assert_eq!("", find_first_word(""));
    }
}