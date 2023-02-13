use crate::prs_data_types;
use std::fs;

use serde_json::from_str;

pub fn load_data() -> Option<prs_data_types::Root> {
    let contents: String = fs::read_to_string("./data/nzprsBackup.json")
        .expect("Should have been able to read the file");
    let prs_data = from_str(contents.as_str());

    match prs_data {
        Ok(root) => root,
        Err(_) => None,
    }
}
