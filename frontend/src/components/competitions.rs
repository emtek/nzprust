use crate::{
    components::progress_bar::Progress, data::prs_data_types::Competition, data::*,
    routes::AppRoute,
};
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::Link;

async fn get_competitions() -> Result<Vec<Competition>, MultiError> {
    get_data("/competitions".to_string()).await
}

#[function_component(CompetitionList)]
pub fn competition_list() -> Html {
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
            <Progress/>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct CompetitionDetailProps {
    pub id: String,
}

async fn get_competition(id: String) -> Result<Competition, MultiError> {
    get_data(format!("/competition/{}", id)).await
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
                    <span>{"  "}</span>
                    <span>{ &competition.comp_date }</span>
                    </p>
                    <p class="sub-title">
                    <ion-icon name="location"/>
                    <span>{"  "}</span>
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
            <Progress/>
        }
    }
}
