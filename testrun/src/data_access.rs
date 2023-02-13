use crate::prs_data_types;
use std::fs;

use anyhow::Result;
use serde_json::from_str;

pub fn load_data() -> Result<prs_data_types::Root> {
    let contents: String = fs::read_to_string("./data/nzprsBackup.json")?;
    let r = from_str(&contents)?;
    Ok(r)
}
