fn addition(v1: u32, v2: u32) -> u32 {
    v1 + v2
}

fn main() {
    let mut should_stop = false;
    while should_stop == false {
        println!("What is the best programming language ever : ");
        let line: String = text_io::read!("{}\n");
        if line == String::from("rust") {
            should_stop = true;
        } else {
            println!("Try again");
        }
    }
    println!("Good boy/girl !")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_addition() {
        assert_eq!(addition(1, 1), 2);
    }
}
