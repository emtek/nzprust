use std::future::Future;

use crate::{
    data::prs_data_types::{self, Ranking},
    routes::AppRoute,
};
use chrono::{Months, NaiveDate};
use reqwest;
use serde_json::{self, from_str};
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::{use_route, Link, Redirect};

#[derive(Properties, PartialEq)]
pub struct RankingDetailProps {
    pub date: String,
}

async fn get_ranking(date: String) -> Result<Ranking, MultiError> {
    let r = reqwest::get(format!("http://127.0.0.1:8000/ranking/{}", date))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let ranking = from_str::<Ranking>(&r);
    match ranking {
        Ok(ranking) => Ok(ranking),
        Err(_) => Err(MultiError::DeserializeError),
    }
}

fn next_ranking(date: String) -> String {
    let current_date = date.parse::<NaiveDate>();
    match current_date {
        Ok(current_date) => current_date
            .checked_add_months(Months::new(1))
            .unwrap()
            .format("%Y-%m-%d")
            .to_string(),
        Err(_) => date,
    }
}

fn previous_ranking(date: String) -> String {
    let current_date = date.parse::<NaiveDate>();
    match current_date {
        Ok(current_date) => current_date
            .checked_sub_months(Months::new(1))
            .unwrap()
            .format("%Y-%m-%d")
            .to_string(),
        Err(_) => date,
    }
}

#[function_component(RankingDetail)]
pub fn ranking_detail(props: &RankingDetailProps) -> Html {
    let date = props.date.clone();
    let initial_date = date.clone();
    let next_date = next_ranking(date.clone());
    let prev_date = previous_ranking(date.clone());
    let now = chrono::Utc::now().format("%Y-%m-01").to_string();
    let is_current = now == date;
    let active = match is_current {
        true => Some("is-active"),
        false => None,
    };
    let ranking_request = use_async(async move { get_ranking(date.clone()).await });
    if ranking_request.loading {
        html! {
            <div class="container">
                <div class="is-one-third">
                    <progress class="progress is-info" max="100"></progress>
                </div>
            </div>
        }
    } else {
        if let Some(ranking) = &ranking_request.data {
            if ranking.date != initial_date {
                if !ranking_request.loading {
                    ranking_request.run();
                }
            }
            html! {
                <>
                <section class="hero is-info">
                    <div class="hero-body">
                        <p class="title">
                        {"Ranking"}
                        </p>
                        <p class="sub-title">
                        {&ranking.date}
                        </p>
                    </div>
                    <div class="hero-foot">
                        <nav class="tabs">
                        <div class="container">
                            <ul>
                            <li class={classes!(active)}>
                            <Link<AppRoute> to={AppRoute::RankingDetail { date: now}}>
                            {"Current"}</Link<AppRoute>></li>
                            <li>
                                <Link<AppRoute> to={AppRoute::RankingDetail { date: prev_date}}>
                                {"Previous month"}</Link<AppRoute>></li>
                            <li class={classes!(active)}>
                                <Link<AppRoute> to={AppRoute::RankingDetail { date: next_date}}>
                                {"Next month"}</Link<AppRoute>>
                            </li>
                            </ul>
                        </div>
                        </nav>
                    </div>
                </section>
                <section class="section">
                    <table class="table is-fullwidth">
                    <thead>
                        <tr>
                        <th><abbr title="Rank">{"Rank"}</abbr></th>
                        <th>{"Name"}</th>
                        <th><abbr title="Points">{"Points"}</abbr></th>
                        <th class="is-hidden-mobile"><abbr title="Competitions that go into the ranking">{"Competitions"}</abbr></th>
                        </tr>
                    </thead>
                    <tbody>
                    {
                        ranking.ranking_points.iter().enumerate().map(|(i, ranking_point)|
                            html!{
                            <tr>
                                <td>{i+1}</td>
                                <td><strong>
                                <Link<AppRoute> to={AppRoute::PilotDetail {pin: ranking_point.pilot_pin.clone() }}>
                                {format!("{} {} ", &ranking_point.pilot_first_name, &ranking_point.pilot_last_name)}
                                {
                                    if let Some(gender) = &ranking_point.pilot_gender {
                                        if gender == "MALE" { html!{<ion-icon class="has-text-info-dark" name="male"></ion-icon>} }
                                        else{ html!{<ion-icon class="has-text-danger-dark" name="female"></ion-icon>}}
                                    }else{html!{<></>}}
                                }</Link<AppRoute>>
                                </strong>
                                </td>
                                <td>{format!("{:.2}", &ranking_point.total_points)}</td>
                                <td class="is-hidden-mobile">
                                <table class="table">
                                <tbody>
                                {ranking_point.results.iter().map(|result| html!{
                                <tr>
                                        <td>{&result.place}</td>
                                        <td>{format!("{:.2}", &result.points)}</td>
                                        <td>{&result.comp_name}</td>
                                </tr>
                                }).collect::<Html>()}
                                </tbody>
                                </table>
                                </td>
                            </tr>
                        }).collect::<Html>()
                    }
                    </tbody>
                    </table>
                </section>
                </>
            }
        } else {
            ranking_request.run();
            html! { <></>}
        }
    }
}

// You can use thiserror to define your errors.
#[derive(Clone, Debug, PartialEq)]
enum MultiError {
    DeserializeError,
    // etc.
}
