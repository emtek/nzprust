use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};
use frontend::prs_data_types::{Competition, CompetitionPilot, Placing};
use reqwest::StatusCode;
use scraper::Selector;

use crate::data::{
    data_access::{get_data_external, get_html_external},
    highcloud_data_types::HighCloudRoot,
};

pub async fn from_highcloud(Path(comp_id): Path<i32>) -> Response {
    if let Ok(data) = get_data_external::<HighCloudRoot>(format!(
        "http://xc.highcloud.net/get_result.php?comPk={}&_=1678092363685",
        comp_id
    ))
    .await
    {
        let mut tasks = 0;
        if let Some(first) = data.data.first() {
            tasks = first
                .iter()
                .skip(10)
                .take_while(|a| a.is_i64() || a.as_str().unwrap_or("").cmp("").is_ne())
                .count();
        }
        (
            StatusCode::OK,
            Json(&Competition {
                name: data.compinfo.com_name,
                location: data.compinfo.com_location,
                comp_date: data.compinfo.com_date_from,
                num_tasks: tasks as i64,
                placings: data
                    .data
                    .iter()
                    .map(|v| Placing {
                        place: v.get(0).unwrap().as_i64().unwrap(),
                        pilot: CompetitionPilot {
                            pin: v.get(1).unwrap().as_str().unwrap().to_string(),
                            first_name: v.get(3).unwrap().as_str().unwrap().to_string(),
                            last_name: v.get(3).unwrap().as_str().unwrap().to_string(),
                            gender: v.get(5).unwrap().as_str().unwrap().to_string(),
                        },
                        ..Default::default()
                    })
                    .collect(),
                ..Default::default()
            }),
        )
            .into_response()
    } else {
        (StatusCode::NOT_FOUND).into_response()
    }
}

pub async fn from_fai(Path(comp_id): Path<i32>) -> Response {
    if let Ok(data) = get_html_external(format!(
        "https://civlcomps.org/ranking/paragliding-xc/competition?id={}",
        comp_id
    ))
    .await
    {
        let ranking = Selector::parse(".pilot-item").unwrap();
        let table_data = Selector::parse("td").unwrap();
        let link = Selector::parse("a").unwrap();
        let text = data
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
            .filter(|p| p[6].contains("NZL"))
            .map(|f| Placing {
                id: 1,
                pilot: CompetitionPilot {
                    pin: f[7].to_owned(),
                    first_name: f[5].to_owned(),
                    last_name: f[5].to_owned(),
                    ..Default::default()
                },
                place: f[0].parse::<i64>().unwrap(),
                fai_points: f[2].parse::<f64>().unwrap(),
                ..Default::default()
            })
            .collect();
        let first_table = Selector::parse("#tableMain>tbody>tr>td").unwrap();
        let header = Selector::parse(".header-rankings h2").unwrap();
        let comp_name = data
            .select(&header)
            .map(|h| h.inner_html())
            .collect::<Vec<String>>();
        let comp_date = data
            .select(&first_table)
            .take(1)
            .map(|h| h.inner_html())
            .collect::<Vec<String>>();
        (
            StatusCode::OK,
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
            }),
        )
            .into_response()
    } else {
        (StatusCode::NOT_FOUND).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn from_highcloud_should_return_result() {
        let result = from_highcloud(Path(358)).await;
        assert_eq!(result.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn from_fai_works() {
        let response = from_fai(Path(5859)).await;
        assert_eq!(response.status(), StatusCode::OK)
    }
}
