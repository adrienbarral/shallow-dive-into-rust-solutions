use std::{sync::{Arc, Mutex}, thread::{self, spawn}, time::Duration};

fn main() {

    let increment = Arc::new(Mutex::new(1));
    let must_exit = Arc::new(Mutex::new(false));
    let mut handles = vec![];

    let increment_clone = increment.clone();
    let must_exit_clone = must_exit.clone();
    let incrementor = spawn(move ||{
        let mut counter = 0;
        loop {
            let inc = *increment_clone.lock().unwrap();
            counter += inc;
            println!("Incremented: {}", counter);
            thread::sleep(Duration::from_millis(1000));
            if *must_exit_clone.lock().unwrap() {
                break;
            }
        }
    });

    let input = spawn(move ||{
        loop {
            let mut input = String::new();
            if std::io::stdin().read_line(&mut input).is_ok() {
                let input = input.trim();
                if input == "exit" {
                    *must_exit.lock().unwrap() = true;
                    break;
                }
                let inc = input.parse::<i32>().unwrap();
                *increment.lock().unwrap() = inc;    
            }
        }
    });
    handles.push(input);
    handles.push(incrementor);

    handles.into_iter().for_each(|h| h.join().unwrap());
}
