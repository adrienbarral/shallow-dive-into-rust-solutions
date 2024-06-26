use chrono::{Datelike, Local};

fn is_weekend_soon_if(day: chrono::Weekday) {
    if day == chrono::Weekday::Fri {
        println!("Yes !");
    } else if day == chrono::Weekday::Sat || day == chrono::Weekday::Sun {
        println!("We are in week end !")
    } else {
        println!("NO !");
    }
}

fn is_weekend_soon_match(day: chrono::Weekday) {
    match day {
        chrono::Weekday::Fri => println!("Yes !"),
        chrono::Weekday::Sat | chrono::Weekday::Sun => println!("We are in week end !"),
        _ => println!("NO !"),
    }
}

fn main() {
    let day = Local::now().weekday();

    is_weekend_soon_match(day);
}
