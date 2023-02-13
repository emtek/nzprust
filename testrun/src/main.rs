mod constants;
mod data_access;
mod prs_data_types;
mod rankings;

use anyhow::{Result, bail};
use chrono::NaiveDate;


fn main() -> Result<()> {
    println!("{}", std::env::current_dir().unwrap().to_str().unwrap());

    let root = data_access::load_data()?;

    let rank_date = &root.rankings[0].date.parse::<NaiveDate>()?;
    let Some(ranks) = rankings::calculate_rankings(rank_date, &root.competitions) else {
        bail!("Failed to calculate new rankings");
    };

    for r in ranks {
        println!("{:?}", r)
    }
    Ok(())
}
