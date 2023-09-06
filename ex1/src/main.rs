fn addition(v1: u32, v2: u32) -> u32 {
    v1 + v2
}

fn main() {    
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_addition() {
        assert_eq!(addition(1,1), 2);
    }
}