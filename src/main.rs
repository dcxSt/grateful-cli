use chrono;
use clap::Parser;
use io::Write;
use json;
use std::io;
use log;
mod utils;
mod dateutils;


/// Simple program to greet a person
#[derive(Parser)]
struct Cli {
    /// The pattern to look for, this is the only argument, something like "history" or "last"
    pattern: String,
}

// returns true if the last entry was today 
/// Check to see if the last time you ran this command was today or yesterday
fn check_last_entry_today() -> bool {
    let mut grateful: json::JsonValue = get_json();
    let last_entry: json::JsonValue = grateful["grateful"].pop();
    let date_str = last_entry[0].dump().replace("\"","");
    let today_str = chrono::offset::Local::today().to_string();
    log::trace!("{} , {}", date_str.as_str(), today_str.as_str());
    if date_str.as_str() == today_str.as_str() {
        log::trace!("{} == {} is true", date_str.as_str(), today_str.as_str());
        return true;
    }
    false
}

// gets user input, adds it to the json
/// Interactive user input loop
/// Obtain three gratefulness entries from the user and return them
fn get_user_input() -> Vec<String> {
    let datestring = dateutils::get_today_string(); // chrono::offset::Local::today().to_string();
    let mut entry = vec![datestring];

    let mut buffer = String::new();
    let stdin = io::stdin();
    for n in 0..3 {
        buffer.clear();
        while buffer.as_str() == "" || buffer.as_str() == "\n" {
            buffer.clear();
            print!("What are you greatful for today? ({})> ", 3 - n);
            io::stdout().flush().expect("flush failed");
            stdin.read_line(&mut buffer).unwrap();
        }
        // entry.push(buffer.to_string());
        entry.push(buffer.trim_end_matches("\n").to_string());
    }
    return entry;
}

fn verify_format(grateful: &json::JsonValue) {
    for idx in 0..grateful["grateful"].len() {
        assert!(grateful["grateful"][idx].len() == 4 , "grateful.json has been corrupted");
    }
}

fn get_json() -> json::JsonValue {
    // if there is no directory create it
    utils::create_grateful_dir();
    // if there is no grateful.json, create it
    utils::init_file(utils::get_grateful_json_path().as_str()).unwrap();
    match std::fs::read_to_string(utils::get_grateful_json_path().as_str()) {
        Ok(data_str) => {
            let data = json::parse(data_str.as_str()).unwrap();
            verify_format(&data); // checks that grateful.json is well formatted
            data
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}

// this is the main data entry loop, to be run max once per day
fn add_grateful_entry() -> io::Result<()> {
    let entry: Vec<String> = get_user_input();

    let mut grateful: json::JsonValue = get_json();
    grateful["grateful"].push(entry).unwrap();

    // write to file
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(utils::get_grateful_json_path().as_str())?;
    f.write_all(json::stringify_pretty(grateful, 4u16).as_bytes())?;
    f.flush()?;

    Ok(())
}



/// Display the history of grateful entries
fn display_history(grateful: &json::JsonValue , size: Option<usize> ) {
    let len = grateful["grateful"].len();
    let mut size:usize = size.unwrap_or(len); // if size is None just display the whole history
    if size > len {
        size = len; // if the user enters a large number, just display the max
    }
    for idx in (len-size)..len {
        let entry = &grateful["grateful"][idx];
        let date_string = &entry[0];
        println!("{}", dateutils::date_string_pretty(date_string.to_string()));
        for msg_idx in 1..4 {
            println!("({}) {}", 4-msg_idx, entry[msg_idx]);
        }
        println!(); // empty line, for display formatting purposes
        // println!("{}" , grateful["grateful"][idx]);
    }
}

/// Main script, parse user's input and execute commands
fn main() -> io::Result<()> {
    match Cli::try_parse() {
        Ok(r) => {
            if r.pattern == "history".to_string() {
                let grateful: json::JsonValue = get_json();
                display_history(&grateful , None);
            } else if r.pattern == "last".to_string() {
                let grateful: json::JsonValue = get_json();
                display_history(&grateful , Some(1));
            } else {
                println!("Oops, {} is not a valid pattern.\nTry `grateful history` or `grateful last` instead", r.pattern);
            }
        }
        Err(_) => {
            if check_last_entry_today() {
                println!("You've already written what you're grateful for today.");
                println!("You can view your history of gratefulness with `grateful history`");
                println!("Ending program, see ya tomorow!");
            } else {
                add_grateful_entry()?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_json() {
        let _jfile : json::JsonValue = get_json();
    }

    #[test]
    fn test_display_history() {
        let grateful = get_json();
        println!("Calling display history a bunch."); // `cargo test -- --nocapture`
        println!("huge number -> should display all");
        display_history(&grateful , Some(10000000000000));
        println!("zero -> should display nothing");
        display_history(&grateful , Some(0));
        println!("1");
        display_history(&grateful , Some(1));
        println!("None -> should display all");
        display_history(&grateful , None);
        println!("2");
        display_history(&grateful , Some(2));
    }
}



