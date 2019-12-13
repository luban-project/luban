
use std::fs;
use std::io;

pub fn read_to_string(path: &str) -> Option<String> {
    match fs::read_to_string(path) {
        Ok(content) => Some(content),
        Err(why) => {
            println!("warning: failed to file {} : {}", path, why);
            None
        }
    }
}



