use std::fs;
mod constants;
mod data_access;
mod prs_data_types;
mod rankings;
use serde_json::from_str;

fn main() {
    println!("{}", std::env::current_dir().unwrap().to_str().unwrap());
    let contents: String = fs::read_to_string("./data/nzprsBackup.json")
        .expect("Should have been able to read the file");
}
