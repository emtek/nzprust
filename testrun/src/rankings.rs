use crate::constants::constants;
use crate::prs_data_types::{
    CompResult, Competition, Pilot, Pilot2, Placing, Ranking, RankingPoint, Root,
};
use crate::{data_access, prs_data_types};
use chrono::prelude::*;
use chrono::{DateTime, FixedOffset, Months, ParseError, Utc};
use std::cmp::min;
use std::collections::HashMap;
use std::hash::Hash;
use std::num;
use std::ops::Div;
use std::thread::current;

use serde_json::from_str;

fn participant_number(
    current_competition: Competition,
    competition_history: Vec<Competition>,
) -> Option<f64> {
    // Get the date 2 years prior to the comp date
    let this_comp_date = current_competition.comp_date.parse::<NaiveDate>().ok()?;
    let two_years_earlier = this_comp_date.checked_sub_months(Months::new(24))?;
    println!("{}", current_competition.pq.to_string());
    // Number of participants in this comp
    let num_participants = current_competition.placings.len() as f64;

    // Calc the average num participants in the last 24 months
    let previous_competitions: Vec<f64> = competition_history
        .iter()
        .filter(|c| {
            let other_comp_date = c.comp_date.parse::<NaiveDate>();
            match other_comp_date {
                Ok(other_comp_date) => {
                    !c.overseas
                        && other_comp_date.lt(&this_comp_date)
                        && other_comp_date.gt(&two_years_earlier)
                }
                Err(_) => false,
            }
        })
        .map(|c| c.placings.len() as f64)
        .collect();
    let previous_count = previous_competitions.len() as f64;
    let ave_num_participants: f64 = previous_competitions.iter().sum::<f64>() / previous_count;
    let raw_pn = (num_participants / ave_num_participants).sqrt();
    Some(raw_pn.min(constants::PN_MAX))
}

fn pilot_quality(ranking: Option<&Ranking>, placings: Vec<Placing>) -> f64 {
    match ranking {
        None => 1.0,
        Some(ranking) => {
            // Get the most recent ranking points prior to this comp for these pilots and for top pilots.
            let Pq_srp = Pq_srp(
                placings.iter().map(|p| p.pilot.clone()).collect(),
                ranking.ranking_points.clone(),
            );
            let Pq_srtp = Pq_srtp(
                (placings.len() as f64 / 2.0).round(),
                ranking.ranking_points.clone(),
            );
            Pq_srp / Pq_srtp * (1.0 - constants::PQ_MIN) + constants::PQ_MIN
        }
    }
}

fn Pq_srtp(num: f64, ranking_points: Vec<RankingPoint>) -> f64 {
    ranking_points
        .iter()
        .map(|rp| rp.total_points)
        .take(num as usize)
        .sum()
}
/// Pilot quality
fn Pq_srp(pilots: Vec<Pilot2>, ranking_points: Vec<RankingPoint>) -> f64 {
    let mut points: Vec<f64> = ranking_points
        .iter()
        .filter(|rp| pilots.iter().any(|p| p.pin == rp.pilot_pin))
        .map(|rp| rp.total_points)
        .collect();
    points.sort_by(|a, b| b.total_cmp(a));

    points
        .iter()
        .take((points.len() as f64 / 2.0).round() as usize)
        .sum()
}
/// Get the quality of the competition
fn competition_task_quality(number_of_tasks: u8) -> f64 {
    match number_of_tasks {
        0 => 0.0,
        1 => 0.4,
        2 => 0.6,
        3 => 0.8,
        4 => 0.9,
        _ => 1.0,
    }
}
/// Get the competition decay factor
fn competition_decay(days_since_competition: f64) -> f64 {
    let n = days_since_competition / constants::TD_PERIOD * constants::TD_B - constants::TD_B / 2.0;
    1.0 / (1.0 + constants::TD_A.powf(n))
}
/// Calculate the decayed rankings for a date given past competition results
fn calculate_rankings(
    ranking_date: &NaiveDate,
    competitions: &Vec<Competition>,
) -> Option<Vec<RankingPoint>> {
    // Get the date 3 years prior to the ranking date
    let three_years_earlier = ranking_date.checked_sub_months(Months::new(36))?;

    Some(
        competitions
            .iter()
            // Cycle through each comp within the last 3 years
            .filter(|c| {
                let other_comp_date = c.comp_date.parse::<NaiveDate>();
                match other_comp_date {
                    Ok(date) => date.gt(&three_years_earlier) && date.lt(ranking_date),
                    _ => false,
                }
            })
            .flat_map(|competition| {
                competition
                    .placings
                    .iter()
                    .map(|placing| time_decayed_points(competition, placing, ranking_date))
                    .flatten()
            })
            .fold(
                HashMap::new(),
                |mut pin_results: HashMap<String, Vec<CompResult>>,
                 pin_result: (String, CompResult)| {
                    match pin_results.get_mut(&pin_result.0) {
                        Some(results) => {
                            results.push(pin_result.1.clone());
                            results.sort_by(|a, b| b.points.total_cmp(&a.points));
                            remove_extra_overseas(results);
                        }
                        None => {
                            pin_results.insert(pin_result.0, [pin_result.1.clone()].to_vec());
                        }
                    };
                    pin_results
                },
            )
            .iter()
            .map(|pin_results| RankingPoint {
                pilot_first_name: pin_results.0.clone(),
                pilot_gender: None,
                pilot_last_name: pin_results.0.clone(),
                pilot_pin: pin_results.0.clone(),
                results: pin_results.1.clone(),
                total_points: pin_results.1.iter().take(4).map(|r| r.points).sum(),
            })
            .collect(),
    )
}

/// Remove any extra overseas competitions keeping the most valuable 2
fn remove_extra_overseas(results: &mut Vec<CompResult>) {
    let overseas_results: Vec<&CompResult> = results.iter().filter(|r| r.overseas).collect();
    if overseas_results.len() > 2 {
        if let Some(position) = results
            .iter()
            .position(|f| f.comp_id == overseas_results.last().unwrap().comp_id)
        {
            results.remove(position);
        }
    }
}

/// Calculate the devalued points for the competitions
fn time_decayed_points(
    competition: &Competition,
    placing: &Placing,
    ranking_date: &NaiveDate,
) -> Option<(String, CompResult)> {
    let comp_date = competition.comp_date.parse::<NaiveDate>().ok()?;
    let days_since_competition = ranking_date.signed_duration_since(comp_date).num_days() as f64;
    Some((
        placing.pilot.pin.clone(),
        CompResult {
            place: placing.place.clone(),
            comp_id: competition.id.clone(),
            comp_name: competition.name.clone(),
            points: placing.points * competition_decay(days_since_competition),
            overseas: competition.overseas.clone(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn recalculate_should_get_number() {
        let data = data_access::load_data();
        if let Some(mut root) = data {
            let pn = participant_number(root.competitions[9].clone(), root.competitions.clone());
            let pq = pilot_quality(root.rankings.first(), root.competitions[9].clone().placings);
            if let Some(pn) = pn {
                println!("{}", pn.abs());
            }
            println!("{}", pq.abs());
        }
    }

    #[test]
    fn recalculate_should_get_good() {
        let data = data_access::load_data();
        if let Some(mut root) = data {
            let pn = calculate_rankings(
                &root.rankings[0].date.parse::<NaiveDate>().unwrap(),
                &root.competitions,
            );
            println!("{} {}", &root.rankings[0].id, &root.rankings[0].date);
            if let Some(mut pn) = pn {
                pn.sort_by(|a, b| a.pilot_pin.cmp(&b.pilot_pin));
                for point in pn {
                    println!("{} {}", point.pilot_pin, point.total_points);
                }
            }
        }
    }

    #[test]
    fn competitions_should_decay() {
        assert_eq!(competition_decay(10.0), 0.99889299013837107);
        assert_eq!(competition_decay(549.0), 0.49683787436410787);
        assert_eq!(competition_decay(1086.0), 0.0011070098616289667);
    }
}
