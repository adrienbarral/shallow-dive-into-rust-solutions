fn say_hello(name: &String) {
    println!("Hello  {} ! ", name);
}

fn get_full_name(first_name: &String, given_name: &String) -> String {
    // Look into the documentation of String, it may exists something to append a string to another.

    let full_name = [first_name.clone(), given_name.clone()].join(" ");
    let full_name2 = [first_name, " ", given_name].concat();
    let mut res = first_name.clone();
    res.push_str(" ");
    res.push_str(given_name);
    return res;
}

fn main() {
    println!("Enter your First Name : ");
    let first_name: String = text_io::read!("{}\n");

    say_hello(&first_name);

    // Exercice : Uncomment this and make this program compile

    println!("Enter your Given Name : ");
    let given_name: String = text_io::read!("{}\n");

    let full_name = get_full_name(&first_name, &given_name);

    println!("Nice to meet you {}", full_name);
}

#[cfg(test)]
mod tests {
    use crate::get_full_name;

    #[test]
    fn ex3_can_generate_full_name() {
        // Fill this test... I want to check that given a first name : BARRAL, and a given name Adrien, the function return "BARRAL Adrien"
        assert_eq!(
            get_full_name(&String::from("BARRAL"), &String::from("Adrien")),
            "BARRAL Adrien"
        );
    }
}
