use chrono;
use chrono::NaiveDate;
use chrono::Datelike;

// for converting month integers to strings, it's for display purposes
fn monthstr(month_int:u32) -> &'static str {
    match month_int {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => panic!("Not valid month"),
    }
}

/// Creates an ugly date string for serialization
pub fn get_today_string() -> String {
    chrono::offset::Local::today().to_string()
}

/// Converts ugly date string into a pretty date string, 
/// for displaying to user
pub fn date_string_pretty(date_string: String) -> String {
    let d = NaiveDate::parse_from_str(date_string.as_str() , "%Y-%m-%d%z").unwrap();
    format!("{} {} {} {}", d.weekday(), monthstr(d.month()), d.day(), d.year())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_today_string() {
        println!("get date {}", get_today_string()); // `cargo test -- --nocapture`
    }

    #[test]
    fn test_datetime() {
        let d = NaiveDate::parse_from_str("2022-02-07-05:00", "%Y-%m-%d%z").unwrap();
        assert_eq!(format!("{}", d.year()) , "2022");
        // to display print statememnts use `cargo test -- --nocapture`
        println!("month = {}", d.month());
        println!("month str = {}", monthstr(d.month()));
        println!("weekday = {}", d.weekday());
        println!("day = {}", d.day());
        assert_eq!(format!("{}", monthstr(d.month())) , "February");
    }

    #[test]
    fn test_date_string_pretty() {
        println!("test_date_string_pretty -> {}", date_string_pretty(get_today_string())); // `cargo test -- --nocapture` to display
    }
}




