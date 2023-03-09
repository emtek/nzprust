use crate::constants::constants;
use crate::prs_data_types::{CompResult, Competition, Pilot2, Placing, Ranking, RankingPoint};

use chrono::prelude::*;
use chrono::Months;
use serde_json::json;
use std::collections::HashMap;

fn participant_number(
    current_competition: &Competition,
    competition_history: &Vec<Competition>,
) -> Option<f64> {
    // Get the date 2 years prior to the comp date
    let this_comp_date = current_competition.comp_date.parse::<NaiveDate>().ok()?;
    let two_years_earlier = this_comp_date.checked_sub_months(Months::new(24))?;
    // Number of participants in this comp
    let num_participants = current_competition.placings.len() as f64;

    // Calc the average num participants in the last 24 months
    let previous_competition_placings: Vec<f64> = competition_history
        .iter()
        .map(|previous_competition| {
            if let Ok(other_comp_date) = previous_competition.comp_date.parse::<NaiveDate>() {
                if !previous_competition.overseas // Exclude Overseas comps from the average because we just want the average num pilots at NZ comps
                        && other_comp_date.lt(&this_comp_date)
                        && other_comp_date.gt(&two_years_earlier)
                {
                    return Some(previous_competition.placings.len() as f64);
                }
            }
            None
        })
        .flatten()
        .collect();
    let previous_competition_count = previous_competition_placings.len() as f64 + 1.0;
    let ave_num_participants: f64 = (previous_competition_placings.iter().sum::<f64>()
        + num_participants)
        / previous_competition_count;
    println!("{} / {}", num_participants, ave_num_participants);
    let raw_pn = (num_participants / ave_num_participants).sqrt();
    Some(raw_pn.min(constants::PN_MAX))
}

/// .
pub fn recalculate_competition(
    competition: &Competition,
    ranking: Option<&Ranking>,
    comps: &Vec<Competition>,
) -> Option<Competition> {
    let mut updated_competition = competition.clone();
    let pq = pilot_quality(ranking, &competition.placings);
    updated_competition.pq = json!(pq);
    println!("{}", pq);
    updated_competition.pn = participant_number(competition, comps)?;
    let mut max_points = 0.0;
    for mut placing in updated_competition.placings.iter_mut() {
        if competition.overseas {
            placing.points = placing.fai_points * competition.exchange_rate;
        } else {
            placing.pplacing = calculate_pilot_placing(competition, placing.place as f64);
            placing.pp = placing
                .pplacing
                .powf(1.0 + pq)
                .max(placing.pplacing.powf(2.0));
            placing.points = placing.pp
                * pq
                * updated_competition.pn
                * competition_task_quality(competition.num_tasks as u8)
                * 100.0;
        }
        max_points = placing.points.max(max_points);
    }
    updated_competition.comp_value = max_points;
    Some(updated_competition)
}

fn calculate_pilot_placing(competition: &Competition, place: f64) -> f64 {
    let last_place = competition.placings.len() as f64;
    (last_place - place + 1.0) / last_place
}

fn pilot_quality(ranking: Option<&Ranking>, placings: &Vec<Placing>) -> f64 {
    match ranking {
        None => 1.0,
        Some(ranking) => {
            // Get the most recent ranking points prior to this comp for these pilots and for top pilots.
            let pq_srp = pilot_quality_srp(
                placings.iter().map(|p| p.pilot.clone()).collect(),
                ranking.ranking_points.clone(),
            );
            let pq_srtp = pilot_quality_srtp(
                (placings.len() as f64 / 2.0).round(),
                ranking.ranking_points.clone(),
            );
            if pq_srp == 0.0 || pq_srtp == 0.0 {
                return (1.0 - constants::PQ_MIN) + constants::PQ_MIN;
            }
            pq_srp / pq_srtp * (1.0 - constants::PQ_MIN) + constants::PQ_MIN
        }
    }
}

fn pilot_quality_srtp(num: f64, ranking_points: Vec<RankingPoint>) -> f64 {
    ranking_points
        .iter()
        .map(|rp| rp.total_points)
        .take(num as usize)
        .sum()
}

/// Pilot quality
fn pilot_quality_srp(pilots: Vec<Pilot2>, ranking_points: Vec<RankingPoint>) -> f64 {
    let mut points: Vec<f64> = ranking_points
        .iter()
        .filter(|rp| pilots.iter().any(|p| p.pin.cmp(&rp.pilot_pin).is_eq()))
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
pub fn calculate_rankings(
    ranking_date: &NaiveDate,
    competitions: &Vec<Competition>,
) -> Option<Vec<RankingPoint>> {
    // Get the date 3 years prior to the ranking date
    let three_years_earlier = ranking_date.checked_sub_months(Months::new(36))?;
    let mut rankings: Vec<RankingPoint> = competitions
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
        .collect();
    rankings.sort_by(|a, b| b.total_points.total_cmp(&a.total_points));
    Some(rankings)
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
    use anyhow::Result;
    use serde_json::json;

    use super::*;
    use crate::{data_access, prs_data_types::Pilot};

    #[test]
    fn recalculate_comp_should_get_number() -> Result<()> {
        let mut root = data_access::load_data()?;
        root.competitions
            .sort_by(|a, b| b.comp_date.cmp(&a.comp_date));
        for competition in root.competitions.clone() {
            let comp_val = recalculate_competition(
                &competition,
                root.rankings.iter().find(|r| {
                    let comp_date = competition.comp_date.parse::<NaiveDate>().unwrap();
                    let rdate = &&r.date.parse::<NaiveDate>().unwrap();
                    let two_years_earlier = comp_date.checked_sub_months(Months::new(24)).unwrap();
                    two_years_earlier.lt(&rdate) && (comp_date.gt(&rdate) || comp_date.eq(&rdate))
                }),
                &root.competitions,
            );
            if let Some(comp) = comp_val {
                if (comp.comp_value - competition.comp_value).abs() > 0.00000001 {
                    println!(
                        "{} | {} - {}",
                        competition.name, comp.comp_value, competition.comp_value
                    );
                }
            }
        }
        Ok(())
    }

    #[test]

    fn recalculate_should_get_good() -> Result<()> {
        let root = data_access::load_data()?;
        let pn = calculate_rankings(
            &root.rankings[0].date.parse::<NaiveDate>().unwrap(),
            &root.competitions,
        );
        println!("{} {}", &root.rankings[0].id, &root.rankings[0].date);
        if let Some(mut pn) = pn {
            pn.sort_by(|a, b| a.pilot_pin.cmp(&b.pilot_pin));
            for point in pn {
                for existing_ranking in &root.rankings[0].ranking_points {
                    if existing_ranking.pilot_pin == point.pilot_pin {
                        println!("{} {}", point.pilot_pin, point.total_points);
                        assert_eq!(
                            (point.total_points - existing_ranking.total_points).abs() < 0.00000001,
                            true
                        );
                    }
                }
            }
        }
        Ok(())
    }

    #[test]
    fn competitions_should_decay() {
        assert_eq!(competition_decay(10.0), 0.99889299013837107);
        assert_eq!(competition_decay(549.0), 0.49683787436410787);
        assert_eq!(competition_decay(1086.0), 0.0011070098616289667);
    }

    #[test]
    fn pq_no_ranking() {
        let (_, _, competitions) = get_test_data();
        for comp in &competitions {
            let pq = pilot_quality(None, &comp.placings);
            assert_eq!(pq, 1.0);
        }
    }

    #[test]
    fn pq_with_ranking() {
        let (mut rankings, _, competitions) = get_test_data();
        // Now create a ranking and check again
        let ranking = Ranking {
            id: "2013-09-10".to_string(),
            date: "2013-09-10".to_string(),
            ranking_points: calculate_rankings(
                &"2013-09-10".to_string().parse::<NaiveDate>().unwrap(),
                &competitions,
            )
            .unwrap(),
        };

        rankings.push(ranking);
        let result = recalculate_all(rankings, competitions);
        result.iter().for_each(|c| match c {
            CompetitionOrRanking::Competition(c) => {
                println!("{} {} {}", c.name, c.comp_date, c.pq);
                if c.comp_date == "2013-09-09" {
                    assert_eq!(1.0, c.pq);
                } else if c.comp_date == "2014-10-05" {
                    assert_eq!(1.0, c.pq);
                } else if c.comp_date == "2015-08-03" {
                    assert_eq!(0.7464150943396228, c.pq);
                }
            }
            CompetitionOrRanking::Ranking(r) => {
                for p in &r.ranking_points {
                    println!("{} {} {}", p.pilot_first_name, p.pilot_pin, p.total_points);
                }
            }
        });
    }

    #[test]
    fn test_pn() {
        let (_, _, competitions) = get_test_data();
        assert_eq!(
            1.0,
            participant_number(&competitions[0], &competitions).unwrap()
        );
        assert_eq!(
            0.7669649888473704,
            participant_number(&competitions[1], &competitions).unwrap()
        );
        assert_eq!(
            0.7559289460184544,
            participant_number(&competitions[2], &competitions).unwrap()
        );
    }

    #[allow(dead_code)]
    fn test_pplacing() {

        // assertEquals(0.6666, TestData.pilot1.getPlacingForComp(TestData.auckland).getPplacing(), 0.0001);
        // assertEquals(1.0, TestData.pilot2.getPlacingForComp(TestData.auckland).getPplacing(), 0.0001);
        // assertEquals(0.9166, TestData.pilot3.getPlacingForComp(TestData.auckland).getPplacing(), 0.0001);
        // assertEquals(0.75, TestData.pilot4.getPlacingForComp(TestData.auckland).getPplacing(), 0.0001);
        // assertEquals(0.8333, TestData.pilot5.getPlacingForComp(TestData.auckland).getPplacing(), 0.0001);
        // assertEquals(0.5, TestData.pilot6.getPlacingForComp(TestData.auckland).getPplacing(), 0.0001);
        // assertEquals(0.5833, TestData.pilot7.getPlacingForComp(TestData.auckland).getPplacing(), 0.0001);
        // assertEquals(0.4166, TestData.pilot8.getPlacingForComp(TestData.auckland).getPplacing(), 0.0001);
        // assertEquals(0.3333, TestData.pilot9.getPlacingForComp(TestData.auckland).getPplacing(), 0.0001);
        // assertEquals(0.25, TestData.pilot10.getPlacingForComp(TestData.auckland).getPplacing(), 0.0001);
        // assertEquals(0.1666, TestData.pilot11.getPlacingForComp(TestData.auckland).getPplacing(), 0.0001);
        // assertEquals(0.0833, TestData.pilot12.getPlacingForComp(TestData.auckland).getPplacing(), 0.0001);
        // assertEquals(0.8, TestData.pilot1.getPlacingForComp(TestData.wanaka).getPplacing(), 0.0001);
        // assertEquals(0.6, TestData.pilot2.getPlacingForComp(TestData.wanaka).getPplacing(), 0.0001);
        // assertEquals(1.0, TestData.pilot3.getPlacingForComp(TestData.wanaka).getPplacing(), 0.0001);
        // assertEquals(0.4, TestData.pilot4.getPlacingForComp(TestData.wanaka).getPplacing(), 0.0001);
        // assertEquals(0.2, TestData.pilot5.getPlacingForComp(TestData.wanaka).getPplacing(), 0.0001);
        // assertEquals(1.0, TestData.pilot9.getPlacingForComp(TestData.waikato).getPplacing(), 0.0001);
        // assertEquals(0.75, TestData.pilot12.getPlacingForComp(TestData.waikato).getPplacing(), 0.0001);
        // assertEquals(0.5, TestData.pilot4.getPlacingForComp(TestData.waikato).getPplacing(), 0.0001);
        // assertEquals(0.25, TestData.pilot5.getPlacingForComp(TestData.waikato).getPplacing(), 0.0001);
    }

    #[derive(Clone)]
    enum CompetitionOrRanking {
        Ranking(Ranking),
        Competition(Competition),
    }

    fn recalculate_all(
        rankings: Vec<Ranking>,
        competitions: Vec<Competition>,
    ) -> Vec<CompetitionOrRanking> {
        // Put all comps and rankings into the events tree map so that they
        // can be iterated in chronological order
        let mut joined: Vec<(String, CompetitionOrRanking)> = rankings
            .iter()
            .map(|r| (r.date.clone(), CompetitionOrRanking::Ranking(r.clone())))
            .collect::<Vec<(String, CompetitionOrRanking)>>();
        let mut competition_map = competitions
            .iter()
            .map(|r| {
                (
                    r.comp_date.clone(),
                    CompetitionOrRanking::Competition(r.clone()),
                )
            })
            .collect::<Vec<(String, CompetitionOrRanking)>>();
        joined.append(competition_map.as_mut());
        joined.sort_by(|a, b| a.0.cmp(&b.0));
        let mut previous_ranking: Option<Ranking> = None;
        let mut competition_new = joined
            .iter()
            .map(|f| f.clone())
            .collect::<HashMap<String, CompetitionOrRanking>>();
        for o in joined {
            match o.1 {
                CompetitionOrRanking::Competition(comp) => {
                    let newcomp = recalculate_competition(
                        &comp,
                        previous_ranking.as_ref(),
                        &competition_new
                            .clone()
                            .into_values()
                            .map(|c| match c {
                                CompetitionOrRanking::Competition(comp) => Some(comp),
                                _ => None,
                            })
                            .flatten()
                            .collect(),
                    );

                    if let Some(c) = newcomp {
                        competition_new.insert(
                            c.comp_date.clone(),
                            CompetitionOrRanking::Competition(c.clone()),
                        );
                    }
                }
                CompetitionOrRanking::Ranking(ranking) => {
                    let ranking_points = calculate_rankings(
                        &ranking.date.parse::<NaiveDate>().unwrap(),
                        &competitions,
                    );
                    if let Some(points) = ranking_points {
                        let mut r = ranking.clone();
                        r.ranking_points = points;
                        competition_new
                            .insert(r.date.clone(), CompetitionOrRanking::Ranking(r.clone()));
                        previous_ranking = Some(r.clone());
                    }
                }
            }
        }
        competition_new.clone().into_values().collect()
    }

    fn get_test_data() -> (Vec<Ranking>, Vec<Pilot>, Vec<Competition>) {
        let pilots: Vec<Pilot> = [
            Pilot {
                pin: "1001".to_string(),
                first_name: "First".to_string(),
                last_name: "Pilot".to_string(),
                gender: "None".to_string(),
            },
            Pilot {
                pin: "1002".to_string(),
                first_name: "Second".to_string(),
                last_name: "Pilot".to_string(),
                gender: "None".to_string(),
            },
            Pilot {
                pin: "1003".to_string(),
                first_name: "Third".to_string(),
                last_name: "Pilot".to_string(),
                gender: "None".to_string(),
            },
            Pilot {
                pin: "1004".to_string(),
                first_name: "Fourth".to_string(),
                last_name: "Pilot".to_string(),
                gender: "None".to_string(),
            },
            Pilot {
                pin: "1005".to_string(),
                first_name: "Fifth".to_string(),
                last_name: "Pilot".to_string(),
                gender: "None".to_string(),
            },
            Pilot {
                pin: "1006".to_string(),
                first_name: "Sixth".to_string(),
                last_name: "Pilot".to_string(),
                gender: "None".to_string(),
            },
            Pilot {
                pin: "1007".to_string(),
                first_name: "Seventh".to_string(),
                last_name: "Pilot".to_string(),
                gender: "None".to_string(),
            },
            Pilot {
                pin: "1008".to_string(),
                first_name: "Eighth".to_string(),
                last_name: "Pilot".to_string(),
                gender: "None".to_string(),
            },
            Pilot {
                pin: "1009".to_string(),
                first_name: "Nineth".to_string(),
                last_name: "Pilot".to_string(),
                gender: "None".to_string(),
            },
            Pilot {
                pin: "1010".to_string(),
                first_name: "Tenth".to_string(),
                last_name: "Pilot".to_string(),
                gender: "None".to_string(),
            },
            Pilot {
                pin: "1011".to_string(),
                first_name: "Eleventh".to_string(),
                last_name: "Pilot".to_string(),
                gender: "None".to_string(),
            },
            Pilot {
                pin: "1012".to_string(),
                first_name: "Twelth".to_string(),
                last_name: "Pilot".to_string(),
                gender: "None".to_string(),
            },
        ]
        .to_vec();
        let auck_comp_placing_map: HashMap<String, i64> = HashMap::from_iter([
            ("1001".to_string(), 5),
            ("1002".to_string(), 1),
            ("1003".to_string(), 2),
            ("1004".to_string(), 4),
            ("1005".to_string(), 3),
            ("1006".to_string(), 7),
            ("1007".to_string(), 6),
            ("1008".to_string(), 8),
            ("1009".to_string(), 9),
            ("1010".to_string(), 10),
            ("1011".to_string(), 11),
            ("1012".to_string(), 12),
        ]);
        let wanaka_comp_placing_map: HashMap<String, i64> = HashMap::from_iter([
            ("1001".to_string(), 2),
            ("1002".to_string(), 3),
            ("1003".to_string(), 1),
            ("1004".to_string(), 4),
            ("1005".to_string(), 5),
        ]);
        let waikato_comp_placing_map: HashMap<String, i64> = HashMap::from_iter([
            ("1004".to_string(), 3),
            ("1005".to_string(), 4),
            ("1009".to_string(), 1),
            ("1012".to_string(), 2),
        ]);
        let mut comps: Vec<Competition> = Vec::new();
        let auckland = recalculate_competition(
            &Competition {
                id: "2013-09-09-Auckland".to_string(),
                name: "Auckland Regional Nov 2013".to_string(),
                location: "Auckland".to_string(),
                comp_date: "2013-09-09".to_string(),
                overseas: false,
                exchange_rate: 1.0,
                num_tasks: 1,
                pn: 0.0,
                pq: json!(0.0),
                ave_num_participants: 1.0,
                ta: 0.0,
                comp_value: 0.0,
                td: 0.0,
                placings: pilots
                    .iter()
                    .filter(|p| auck_comp_placing_map.contains_key(&p.pin))
                    .enumerate()
                    .map(|(i, p)| Placing {
                        pilot: Pilot2 {
                            pin: p.pin.clone(),
                            first_name: p.first_name.clone(),
                            last_name: p.last_name.clone(),
                            gender: p.gender.clone(),
                        },
                        place: auck_comp_placing_map[&p.pin],
                        points: 0.0,
                        id: i as i64,
                        pplacing: 0.0,
                        fai_points: 0.0,
                        pp: 0.0,
                    })
                    .collect(),
            },
            None,
            &comps,
        );
        if let Some(comp) = auckland {
            comps.push(comp);
        }
        let wanaka = recalculate_competition(
            &Competition {
                id: "2014-10-05-Wanaka".to_string(),
                name: "Wanaka".to_string(),
                location: "Wanaka".to_string(),
                comp_date: "2014-10-05".to_string(),
                overseas: false,
                exchange_rate: 1.0,
                num_tasks: 2,
                pn: 0.0,
                pq: json!(0.0),
                ave_num_participants: 1.0,
                ta: 0.0,
                comp_value: 0.0,
                td: 0.0,
                placings: pilots
                    .iter()
                    .filter(|p| wanaka_comp_placing_map.contains_key(&p.pin))
                    .enumerate()
                    .map(|(i, p)| Placing {
                        pilot: Pilot2 {
                            pin: p.pin.clone(),
                            first_name: p.first_name.clone(),
                            last_name: p.last_name.clone(),
                            gender: p.gender.clone(),
                        },
                        place: wanaka_comp_placing_map[&p.pin],
                        points: 0.0,
                        id: i as i64,
                        pplacing: 0.0,
                        fai_points: 0.0,
                        pp: 0.0,
                    })
                    .collect(),
            },
            None,
            &comps,
        );
        if let Some(comp) = wanaka {
            comps.push(comp);
        }
        let waikato = recalculate_competition(
            &Competition {
                id: "2015-08-03-Waikato".to_string(),
                name: "Waikato".to_string(),
                location: "Waikato".to_string(),
                comp_date: "2015-08-03".to_string(),
                overseas: false,
                exchange_rate: 1.0,
                num_tasks: 6,
                pn: 0.0,
                pq: json!(0.0),
                ave_num_participants: 1.0,
                ta: 0.0,
                comp_value: 0.0,
                td: 0.0,
                placings: pilots
                    .iter()
                    .filter(|p| waikato_comp_placing_map.contains_key(&p.pin))
                    .enumerate()
                    .map(|(i, p)| Placing {
                        pilot: Pilot2 {
                            pin: p.pin.clone(),
                            first_name: p.first_name.clone(),
                            last_name: p.last_name.clone(),
                            gender: p.gender.clone(),
                        },
                        place: waikato_comp_placing_map[&p.pin],
                        points: 0.0,
                        id: i as i64,
                        pplacing: 0.0,
                        fai_points: 0.0,
                        pp: 0.0,
                    })
                    .collect(),
            },
            None,
            &comps,
        );
        if let Some(comp) = waikato {
            comps.push(comp);
        }
        let ranking = Ranking {
            date: "2013-09-09".to_string(),
            id: "2013-09-09".to_string(),
            ranking_points: calculate_rankings(
                &"2013-09-09".to_string().parse::<NaiveDate>().unwrap(),
                &comps,
            )
            .unwrap(),
        };

        ([ranking].to_vec(), pilots, comps)
    }
}
