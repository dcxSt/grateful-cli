use chrono;
use clap::Parser;
use io::Write;
use json;
use std::io;
// use directories_next::ProjectDirs;
// use grateful_cli::utils;
mod utils;

/// Simple program to greet a person
#[derive(Parser)]
struct Cli {
    /// The pattern to look for, this is the only argument, something like "history" or "last"
    pattern: String,
}

// returns true if the last entry was today 
fn check_last_entry_today() -> bool {
    let mut grateful: json::JsonValue = get_json();
    let last_entry: json::JsonValue = grateful["grateful"].pop();
    let date_str = last_entry[0].dump().replace("\"","");
    let today_str = chrono::offset::Local::today().to_string();
    println!("{} , {}", date_str.as_str(), today_str.as_str());
    if date_str.as_str() == today_str.as_str() {
        println!("{} == {} is true", date_str.as_str(), today_str.as_str());
        return true;
    }
    false
}

// gets user input, adds it to the json
fn grateful_repl() -> Vec<String> {
    let datestring = chrono::offset::Local::today().to_string();
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

fn get_json() -> json::JsonValue {
    // if there is no directory create it
    utils::create_grateful_dir();
    // if there is no grateful.json, create it
    utils::init_file(utils::get_grateful_json_path().as_str()).unwrap();
    match std::fs::read_to_string(utils::get_grateful_json_path().as_str()) {
        Ok(data_str) => {
            let data = json::parse(data_str.as_str()).unwrap();
            data
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}

// this is the main data entry loop, to be run max once per day
fn add_grateful_entry() -> io::Result<()> {
    let entry: Vec<String> = grateful_repl();

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



fn main() -> io::Result<()> {
    // // let mut grateful = object! {
    // //     "grateful": []
    // // };
    // let mut grateful:json::JsonValue = get_json();

    // // let data1 = vec!["2022-02-04", "a", "b", "c"];
    // // let data2 = vec!["2022-02-03", "x", "y", "z"];

    // // grateful["grateful"].push(data1);
    // // grateful["grateful"].push(data2);

    // let entry: Vec<String> = grateful_repl();
    // grateful["grateful"].push(entry);

    // println!("{}", grateful.dump());

    // // grateful = object!{
    // //     "grateful":[]
    // // };

    // // let grateful_str:String = grateful.dump();
    // let mut f = std::fs::OpenOptions::new().write(true).truncate(true).open("./grateful.json")?;
    // f.write_all(json::stringify_pretty(grateful, 4u16).as_bytes())?;
    // f.flush()?;

    // println!("{}", check_last_entry_today());
    // add_grateful_entry()?;

    match Cli::try_parse() {
        Ok(r) => {
            if r.pattern == "history".to_string() {
                let grateful: json::JsonValue = get_json();
                println!("{}", json::stringify_pretty(grateful, 4u16));
            } else if r.pattern == "last".to_string() {
                let grateful: json::JsonValue = get_json();
                let len = grateful["grateful"].len();
                println!("{}", grateful["grateful"][len - 2].to_string());
            } else {
                println!("Oops, {} is not a valid pattern.\nTry `grateful history` or `grateful last` instead", r.pattern);
            }
        }
        Err(_) => {
            if check_last_entry_today() {
                println!("You've already written what you're grateful for today. \nEnding program, see ya tomorow!")
            } else {
                add_grateful_entry()?;
            }
        }
    }

    Ok(())
}

// #[cfg(test)]
// mod tests() {
//     #[test]
// }
