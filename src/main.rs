use chrono::{Utc, TimeZone};
use chrono_tz::{Australia::Sydney,Europe::Amsterdam,Europe::Riga,America::New_York};
use std::time::Duration;
use std::thread::sleep;
use tabled::{Tabled, Table};

#[derive(Tabled)]
struct Location {
    city: String,
    date: String,
    time: String
}

fn main() {
    loop {
        let utc = Utc::now().naive_utc();

        let cities = [
            ("Sydney", Sydney.from_utc_datetime(&utc)),
            ("Riga",Riga.from_utc_datetime(&utc)),
            ("Amsterdam", Amsterdam.from_utc_datetime(&utc)),
            ("New York", New_York.from_utc_datetime(&utc)),
        ];

        let mut location = vec![];

        for city in cities {
            location.push(Location{
                city: city.0.to_string(),
                time: city.1.format("%T").to_string(),
                date: city.1.format("%a %b %e %Y").to_string(),
            });
        };

        let table = Table::new(location).to_string();

        println!("{}",table);
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        sleep(Duration::from_millis(2));
    }
}