use std::future::Future;

use crate::{
    data::prs_data_types::{self, Competition},
    routes::AppRoute,
};
use reqwest;
use serde_json::{self, from_str};
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::Link;

#[derive(Properties, PartialEq)]
pub struct CompetitionListProps {
    pub take: i32,
}

async fn get_competitions() -> Result<Vec<Competition>, MultiError> {
    let r = reqwest::get("http://127.0.0.1:8000/competitions")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let competitions = from_str::<Vec<Competition>>(&r);
    match competitions {
        Ok(competitions) => Ok(competitions),
        Err(_) => Err(MultiError::DeserializeError),
    }
}

#[function_component(CompetitionList)]
pub fn competition_list(props: &CompetitionListProps) -> Html {
    let competitions = use_async(async move { get_competitions().await });

    if let Some(competitions) = &competitions.data {
        html! {
            <>
            <section class="hero is-info">
                <div class="hero-body">
                    <p class="title">
                    {"Competitions"}
                    </p>
                </div>
            </section>
            <section class="section">
                <table class="table is-fullwidth">
                <thead>
                    <tr>
                    <th>{"Name"}</th>
                    <th>{"Date"}</th>
                    <th>{"Location"}</th>
                    <th>{"Tasks"}</th>
                    <th>{"Success Factor (Ta)"}</th>
                    <th>{"Competition Value"}</th>
                    </tr>
                </thead>
                <tbody>
                {
                    competitions.iter().map(|competition|
                        html!{
                        <tr>
                            <td>
                                <Link<AppRoute> to={AppRoute::CompetitionDetail {id: competition.id.clone()}}>
                                    {&competition.name}
                                </Link<AppRoute>>
                            </td>
                            <td>{&competition.comp_date}</td>
                            <td>{&competition.location}</td>
                            <td>{&competition.num_tasks}</td>
                            <td>{&competition.ta}</td>
                            <td>{&competition.comp_value}</td>

                        </tr>
                    }).collect::<Html>()
                }
                </tbody>
                </table>
            </section>
            </>
        }
    } else {
        if !competitions.loading {
            competitions.run();
        }
        html! {
            <div class="container">
                <div class="is-one-third">
                    <progress class="progress is-info" max="100"></progress>
                </div>
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct CompetitionDetailProps {
    pub id: String,
}

async fn get_competition(id: String) -> Result<Competition, MultiError> {
    let r = reqwest::get(format!("http://127.0.0.1:8000/competition/{}", id))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let competitions = from_str::<Competition>(&r);
    match competitions {
        Ok(competitions) => Ok(competitions),
        Err(_) => Err(MultiError::DeserializeError),
    }
}

#[function_component(CompetitionDetail)]
pub fn competition_list(props: &CompetitionDetailProps) -> Html {
    let competition_id = props.id.clone();
    let competition = use_async(async move { get_competition(competition_id).await });

    if let Some(competition) = &competition.data {
        html! {
            <>
            <section class="hero is-info">
                <div class="hero-body">
                    <p class="title">
                    { &competition.name }
                    </p>
                    <p class="sub-title">
                    <ion-icon name="calendar-number-outline"/>
                    <span>{ &competition.comp_date }</span>
                    </p>
                    <p class="sub-title">
                    <ion-icon name="location"/>
                    { &competition.location }
                    </p>
                </div>
            </section>
            <section class="section">
                <table class="table is-fullwidth">
                <thead>
                    <tr>
                    <th>{"Rank"}</th>
                    <th>{"Pilot"}</th>
                    <th>{"Points"}</th>
                    </tr>
                </thead>
                <tbody>
                {
                    competition.placings.iter().map(|placing|
                        html!{
                        <tr>
                            <td>{&placing.place}</td>
                            <td><Link<AppRoute> to={AppRoute::PilotDetail {pin: placing.pilot.pin.clone()}}>
                                {format!("{} {}", &placing.pilot.first_name, &placing.pilot.last_name)}
                            </Link<AppRoute>></td>
                            <td>{format!("{:.2}",&placing.points)}</td>
                        </tr>
                    }).collect::<Html>()
                }
                </tbody>
                </table>
            </section>
            </>
        }
    } else {
        if !competition.loading {
            competition.run();
        }
        html! {
            <div class="container">
                <div class="is-one-third">
                    <progress class="progress is-info" max="100"></progress>
                </div>
            </div>
        }
    }
}

// You can use thiserror to define your errors.
#[derive(Clone, Debug, PartialEq)]
enum MultiError {
    RequestError,
    DeserializeError,
    // etc.
}
