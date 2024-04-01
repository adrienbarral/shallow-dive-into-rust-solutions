struct Person {
    name: String,
    age: u8,
}

impl Ord for Person
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.age.cmp(&other.age)

    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.age == other.age
    }
}

impl Eq for Person {}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_we_can_sort_persons() {
        let simpsons = vec![
            Person {
                name: "Bart".to_string(),
                age: 10,
            },
            Person {
                name: "Homer".to_string(),
                age: 39,
            },
            Person {
                name: "Marge".to_string(),
                age: 38,
            },
            Person {
                name: "Lisa".to_string(),
                age: 8,
            },
            Person {
                name: "Maggie".to_string(),
                age: 1,
            },
        ];

        let mut simpsons = simpsons;
        simpsons.sort();

        simpsons.sort_by(|l, r| l.age.partial_cmp(&r.age).unwrap());
        assert_eq!(simpsons[0].name, "Maggie");
        assert_eq!(simpsons[1].name, "Lisa");
        assert_eq!(simpsons[2].name, "Bart");
        assert_eq!(simpsons[3].name, "Marge");
        assert_eq!(simpsons[4].name, "Homer");
    }
}