use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Json,
};
use frontend::prs_data_types::{Competition, CompetitionPilot, Pilot, Placing, Root};
use polodb_core::Database;
use reqwest::StatusCode;
use scraper::Selector;

use crate::data::{
    data_access::{get_data_external, get_html_external},
    highcloud_data_types::HighCloudRoot,
};

/// Pull a competition from HighCloud and map it. Matching pilots where possible
pub async fn from_highcloud(
    State(data): State<Arc<Database>>,
    Path(comp_id): Path<i32>,
) -> Response {
    if let Ok(highcloud_competition) = get_data_external::<HighCloudRoot>(format!(
        "http://xc.highcloud.net/get_result.php?comPk={}&_=1678092363685",
        comp_id
    ))
    .await
    {
        let mut tasks = 0;
        if let Some(first) = highcloud_competition.data.first() {
            tasks = first
                .iter()
                .skip(10)
                .take_while(|a| a.is_i64() || a.as_str().unwrap_or("").cmp("").is_ne())
                .count();
        }
        Json(&Competition {
            name: highcloud_competition.compinfo.com_name,
            location: highcloud_competition.compinfo.com_location,
            comp_date: highcloud_competition.compinfo.com_date_from,
            num_tasks: tasks as i64,
            placings: highcloud_competition
                .data
                .iter()
                .map(|v| {
                    let fullname = v.get(3).unwrap().as_str().unwrap_or_default().to_string();
                    let mut split_name = fullname.split_whitespace();
                    let first_name = split_name.next();
                    let last_name = split_name.last();
                    let pin = v.get(1).unwrap().as_str().unwrap().to_string();
                    let existing_pilot = search_pilot(&data, &pin.as_str(), &fullname);
                    Placing {
                        place: v.get(0).unwrap().as_i64().unwrap(),
                        pilot: CompetitionPilot {
                            pin: existing_pilot
                                .clone()
                                .map(|p| p.pin)
                                .unwrap_or("".to_string()),
                            first_name: existing_pilot
                                .clone()
                                .map(|p| p.first_name)
                                .unwrap_or(first_name.unwrap_or_default().to_string()),
                            last_name: existing_pilot
                                .clone()
                                .map(|p| p.last_name)
                                .unwrap_or(last_name.unwrap_or_default().to_string()),
                            gender: v.get(5).unwrap().as_str().unwrap().to_string(),
                        },
                        ..Default::default()
                    }
                })
                .collect(),
            ..Default::default()
        })
        .into_response()
    } else {
        (StatusCode::NOT_FOUND).into_response()
    }
}

/// Pull a competition from FAI and map it. Matching pilots where possible
pub async fn from_fai(State(data): State<Arc<Database>>, Path(comp_id): Path<i32>) -> Response {
    if let Ok(html) = get_html_external(format!(
        "https://civlcomps.org/ranking/paragliding-xc/competition?id={}",
        comp_id
    ))
    .await
    {
        let ranking = Selector::parse(".pilot-item").unwrap();
        let table_data = Selector::parse("td").unwrap();
        let link = Selector::parse("a").unwrap();
        let text = html
            .select(&ranking)
            .map(|i| {
                i.select(&table_data)
                    .map(|j| match j.select(&link).next() {
                        Some(link) => format!("{}", link.inner_html()),
                        None => format!("{}", j.inner_html()),
                    })
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();
        let pilots: Vec<Placing> = text
            .iter()
            // We only get NZL pilots from FAI competitions
            .filter(|p| p[6].contains("NZL"))
            .map(|f| {
                let mut split_name = f[5].split_whitespace();
                let first_name = split_name.next();
                let last_name = split_name.last();
                let existing_pilot = search_pilot(&data, &f[7], &f[5]);
                Placing {
                    id: 1,
                    pilot: CompetitionPilot {
                        pin: existing_pilot
                            .clone()
                            .map(|p| p.pin)
                            .unwrap_or("".to_string()),
                        first_name: existing_pilot
                            .clone()
                            .map(|p| p.first_name)
                            .unwrap_or(first_name.unwrap_or_default().to_string()),
                        last_name: existing_pilot
                            .clone()
                            .map(|p| p.last_name)
                            .unwrap_or(last_name.unwrap_or_default().to_string()),
                        ..Default::default()
                    },
                    place: f[0].parse::<i64>().unwrap(),
                    fai_points: f[2].parse::<f64>().unwrap(),
                    ..Default::default()
                }
            })
            .collect();
        let first_table = Selector::parse("#tableMain>tbody>tr>td").unwrap();
        let header = Selector::parse(".header-rankings h2").unwrap();
        let comp_name = html
            .select(&header)
            .map(|h| h.inner_html())
            .collect::<Vec<String>>();
        let comp_date = html
            .select(&first_table)
            .take(1)
            .map(|h| h.inner_html())
            .collect::<Vec<String>>();

        Json(Competition {
            comp_date: comp_date
                .first()
                .unwrap()
                .to_string()
                .split("<br>")
                .take(1)
                .collect::<String>(),
            name: comp_name.first().unwrap().to_string(),
            placings: pilots,
            overseas: true,
            ..Default::default()
        })
        .into_response()
    } else {
        (StatusCode::NOT_FOUND).into_response()
    }
}

/// Find a pilot searching by pin then name
fn search_pilot(data: &Arc<Database>, pin: &str, fullname: &str) -> Option<Pilot> {
    data.collection::<Pilot>("pilots")
        .find(None)
        .unwrap()
        .flatten()
        .collect::<Vec<Pilot>>()
        .iter()
        .find(|p| {
            p.pin.cmp(&pin.to_string()).is_eq()
                || format!(
                    "{} {}",
                    p.first_name.trim().replace("-", ""),
                    p.last_name.trim().replace("-", "")
                )
                .to_lowercase()
                .cmp(&fullname.to_lowercase().trim().replace("-", ""))
                .is_eq()
        })
        .map(|f| f.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::data_access::load_data;

    // #[tokio::test]
    // async fn from_highcloud_should_return_result() {
    //     let result = from_highcloud(State(load_data().unwrap()), Path(358)).await;
    //     assert_eq!(result.status(), StatusCode::OK);
    // }

    // #[tokio::test]
    // async fn from_fai_works() {
    //     let response = from_fai(State(load_data().unwrap()), Path(5859)).await;
    //     assert_eq!(response.status(), StatusCode::OK)
    // }
}
