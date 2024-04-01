use std::{sync::{self, Arc, Mutex}, thread::{self, spawn}, time::Duration};


fn main() {

    let increment = Arc::new(Mutex::new(1));
    let (must_exit_tx, must_exit_rx) = sync::mpsc::channel::<bool>();
    let mut handles = vec![];

    let increment_clone = increment.clone();
    let incrementor = spawn(move ||{
        let mut counter = 0;
        loop {
            let inc = *increment_clone.lock().unwrap();
            counter += inc;
            println!("Incremented: {}", counter);
            if let Ok(exit) = must_exit_rx.recv_timeout(Duration::from_millis(1000)) {
                if exit {
                    return;
                }
            }
        }
    });

    let input = spawn(move ||{
        loop {
            let mut input = String::new();
            if std::io::stdin().read_line(&mut input).is_ok() {
                let input = input.trim();
                if input == "exit" {
                    if must_exit_tx.send(true).is_err() {
                        eprintln!("Error sending exit signal");
                    }
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
