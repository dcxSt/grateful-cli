use std::env;
use std::fs;
use std::io::ErrorKind;
use std::io::Write;
use log;

const GRATEFUL_DIR_NAME: &str = "grateful";

/// Return the value of $HOME or panic if it doesn't exist
pub fn get_home_dir() -> String {
    env::var("HOME").unwrap_or_else(|e| panic!("error getting $HOME env variable: {}", e))
}

/// Create a directory & all parent directories if they don't exist
/// & return the name. Panic if an error occurs while creating the dir
pub fn create_grateful_dir() {
    fs::create_dir_all(&format!("{}/{}", get_home_dir(), &GRATEFUL_DIR_NAME)).unwrap_or_else(|e| {
        // if it already exists, no problem
        if e.kind() != ErrorKind::AlreadyExists {
            panic!("could not create {}/{} directory: {}", get_home_dir(), &GRATEFUL_DIR_NAME, e);
        }
    });
}

/// Open a file for appending or create it if it doesn't exist
/// Panic on error, return the file handle
pub fn init_file(path: &str) -> std::io::Result<()> {
    match std::path::Path::new(get_grateful_json_path().as_str()).exists() {
        true => {
            // don't do anything
            log::trace!("The file {} already exists", &path);
        }
        false => {
            println!("The grateful.json file where we store you grateful data doesn't exist, creating file at {}", &path);
            let mut f = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open(&path)
                .expect(&format!("Error opening {}", &path));
            f.write_all(b"{\"grateful\":[]}")?;
            f.flush()?;
        }
    }
    Ok(()) 
}

pub fn get_grateful_json_path() -> String {
    format!(
        "{}/{}/grateful.json",
        get_home_dir(),
        &GRATEFUL_DIR_NAME
    )
}


