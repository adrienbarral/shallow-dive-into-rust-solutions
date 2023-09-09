use chrono::{DateTime, Datelike, Local};

fn is_weekend_soon_if(day: chrono::Weekday) {
    if day == chrono::Weekday::Thu {
        println!("Yes !");
    } else if day == chrono::Weekday::Sat || day == chrono::Weekday::Sun {
        println!("We are in week end !")
    } else {
        println!("NO !");
    }
}

fn is_weekend_soon_match(day: chrono::Weekday) {
    match day {
        chrono::Weekday::Thu => println!("Yes !"),
        chrono::Weekday::Sat | chrono::Weekday::Sun => println!("We are in week end !"),
        _ => println!("NO !"),
    }
}

fn main() {
    let local: DateTime<Local> = Local::now();
    let day = local.naive_local().weekday();

    is_weekend_soon_match(day);
}
