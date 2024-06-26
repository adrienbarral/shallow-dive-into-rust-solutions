// Exercice : On laisse écrire toute la fonction (avec la signature).
fn find_first_odd_number(numbers: &Vec<i32>) -> Option<i32> {
    for n in numbers {
        if n % 2 != 0 {
            return Some(*n);
        }
    }
    None
}

fn find_first_word(sentence: &str) -> &str {
    let first_word = sentence.split_whitespace().next();
    match first_word {
        Some(word) => word,
        None => sentence
    }
}

fn main() {
}


#[cfg(test)]
mod tests {
    use crate::*;


    #[test]
    fn ex5_can_find_first_odd_number() {
        assert_eq!(None, find_first_odd_number(&vec![0,2,4,6]));
        assert_eq!(Some(1), find_first_odd_number(&vec![0,2,1,6]));
    }

    #[test]
    fn ex5_1_can_find_first_word() {
        assert_eq!("Hello", find_first_word("Hello World"));
        assert_eq!("Hello", find_first_word("Hello"));
        assert_eq!("", find_first_word(""));
    }
}