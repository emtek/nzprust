use crate::{
    components::progress_bar::Progress,
    data::prs_data_types::Ranking,
    data::{prs_data_types::RankingPoint, *},
    routes::AppRoute,
};
use chrono::{Months, NaiveDate};
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::Link;

#[derive(Properties, PartialEq)]
pub struct RankingDetailProps {
    pub date: NaiveDate,
}

async fn get_ranking(date: NaiveDate) -> Result<Ranking, MultiError> {
    get_data(format!("/ranking/{}", date.format("%Y-%m-01").to_string())).await
}

fn next_ranking(date: NaiveDate) -> NaiveDate {
    match date.checked_add_months(Months::new(1)) {
        Some(new_date) => new_date,
        None => date,
    }
}

fn previous_ranking(date: NaiveDate) -> NaiveDate {
    match date.checked_sub_months(Months::new(1)) {
        Some(new_date) => new_date,
        None => date,
    }
}

#[function_component(RankingDetail)]
pub fn ranking_detail(props: &RankingDetailProps) -> Html {
    let date = props.date.clone();
    let initial_date = date.clone();
    let next_date = next_ranking(date.clone());
    let prev_date = previous_ranking(date.clone());
    let now = chrono::Utc::now().date_naive();
    let is_current = now.format("%Y-%m-01").to_string() == date.format("%Y-%m-01").to_string();
    let active = match is_current {
        true => Some("is-active"),
        false => None,
    };
    let hidden = match is_current {
        true => Some("is-hidden"),
        false => None,
    };
    let ranking_request = use_async(async move { get_ranking(date.clone()).await });
    if ranking_request.loading {
        html! {
            <Progress/>
        }
    } else {
        if let Some(ranking) = &ranking_request.data {
            if ranking.date != initial_date.format("%Y-%m-01").to_string() {
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
                        <nav class="tabs is-boxed">
                        <div class="container">
                            <ul>
                            <li class={classes!(active)}>
                                <Link<AppRoute> to={AppRoute::RankingDetail { date: now}}>
                                {"Current"}
                                </Link<AppRoute>>
                            </li>
                            <li>
                                <Link<AppRoute> to={AppRoute::RankingDetail { date: prev_date}}>
                                {"Previous month"}
                                </Link<AppRoute>></li>
                            <li class={classes!(hidden)}>
                                <Link<AppRoute> to={AppRoute::RankingDetail { date: next_date}}>
                                {"Next month"}
                                </Link<AppRoute>>
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
                      ranking_point_list(&ranking.ranking_points)
                    }
                    </tbody>
                    </table>
                </section>
                </>
            }
        } else {
            if let Some(_) = &ranking_request.error {
                return html! { <>
                    <section class="section"><h1 class="title">{"Ranking not found"}</h1>
                    <Link<AppRoute> to={AppRoute::RankingList} >{"Go to the current ranking"}</Link<AppRoute>>
                    </section>
                </>};
            } else {
                ranking_request.run();
            }
            html! { <>
                <Progress/>
            </>}
        }
    }
}

fn ranking_point_list(ranking_points: &Vec<RankingPoint>) -> Html {
    ranking_points.iter().enumerate().map(|(i, ranking_point)|
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
                    <td>
                        <Link<AppRoute> to={AppRoute::CompetitionDetail { id: result.comp_id.clone() }}>
                            {&result.comp_name}
                        </Link<AppRoute>>
                    </td>
            </tr>
            }).collect::<Html>()}
            </tbody>
            </table>
            </td>
        </tr>
    }).collect()
}
