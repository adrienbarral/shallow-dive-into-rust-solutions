use rand::prelude::*;

enum Coin {
    Heads,
    Tails
}

fn flip_coin() -> Coin {
    let mut rng = rand::thread_rng();
    // generate a random boolean following a uniform pdf.
    if rng.gen::<bool>()  {return Coin::Heads;}
    else {return Coin::Tails;}
}

fn main() {
    // Exercice ... write this :
    match flip_coin() {
        Coin::Heads => println!("Heads"),
        Coin::Tails => println!("Tails")
    }
}

