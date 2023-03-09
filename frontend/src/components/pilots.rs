use crate::{
    components::progress_bar::Progress,
    data::prs_data_types::{Competition, Pilot},
    data::*,
    routes::AppRoute,
};
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::Link;

#[derive(Properties, PartialEq)]
pub struct PilotDetailProps {
    pub pin: String,
}

async fn get_pilots() -> Result<Vec<Pilot>, MultiError> {
    get_data("/pilots".to_string()).await
}

async fn get_pilot(pin: String) -> Result<Pilot, MultiError> {
    get_data(format!("/pilot/{}", pin)).await
}

async fn get_pilot_competitions(pin: String) -> Result<Vec<Competition>, MultiError> {
    get_data(format!("/pilot/{}/competitions", pin)).await
}

#[function_component(PilotList)]
pub fn pilot_list() -> Html {
    let pilots = use_async(async move { get_pilots().await });

    if let Some(mut pilots) = pilots.data.clone() {
        pilots.sort_by(|a, b| a.first_name.cmp(&b.first_name));
        html! {
            <>
            <section class="hero is-info">
                <div class="hero-body">
                    <p class="title">
                    {"All pilots"}
                    </p>
                </div>
            </section>
            <section class="section">
                <table class="table is-fullwidth">
                <thead>
                <tr>
                    <td>{"Name"}</td>
                    <td>{"Pin"}</td>
                </tr>
                </thead>
                <tbody>
                {
                    pilots.into_iter().map(|p|
                        html!{
                        <tr>
                            <td>
                                <Link<AppRoute> to={AppRoute::PilotDetail {pin: p.pin.clone()}}>
                                    {format!("{} {} ", p.first_name, p.last_name)}</Link<AppRoute>>
                            </td>
                            <td>{&p.pin}</td>
                        </tr>
                    }).collect::<Html>()
                }
                </tbody>
                </table>
            </section>
            </>
        }
    } else {
        if !pilots.loading {
            pilots.run();
        }
        html! {
            <Progress/>
        }
    }
}

#[function_component(PilotDetail)]
pub fn pilot_detail(props: &PilotDetailProps) -> Html {
    let pin = props.pin.clone();
    let pilot = use_async(async move { get_pilot(pin).await });

    if let Some(pilot) = pilot.data.clone() {
        html! {
            <>
            <section class="hero is-info">
                <div class="hero-body">
                    <p class="title">
                    {format!("{} {} ",pilot.first_name, pilot.last_name)}
                    </p>
                    <p class="sub-title">
                    {&pilot.pin}
                    </p>
                </div>
            </section>
            <section class="section">
                <PilotCompetitionList pin={pilot.pin}/>
            </section>
            </>
        }
    } else {
        if !pilot.loading {
            pilot.run();
        }
        html! {
            <Progress/>
        }
    }
}

#[function_component(PilotCompetitionList)]
pub fn pilot_competition_list(props: &PilotDetailProps) -> Html {
    let pin = props.pin.clone();
    let pilot_pin = pin.clone();
    let competitions = use_async(async move { get_pilot_competitions(pin).await });

    if let Some(competitions) = competitions.data.clone() {
        html! {
            <>

                <table class="table is-fullwidth">
                <thead>
                <tr>
                    <td>{"Rank"}</td>
                    <td>{"Competition"}</td>
                    <td>{"Points"}</td>
                </tr>
                </thead>
                <tbody>
                {
                    competitions.iter().map(|competition| {
                        let placing = &competition.placings.iter().find(|p| p.pilot.pin == pilot_pin).unwrap();
                        html!{
                        <tr>
                            <td>{&placing.place}</td>
                            <td>
                                <Link<AppRoute> to={AppRoute::CompetitionDetail {id: competition.id.clone()}}>
                                    {format!("{}", &competition.name)}
                                </Link<AppRoute>>
                            </td>
                            <td>{format!("{:.2}",&placing.points)}</td>
                        </tr>
                        }
                    }).collect::<Html>()
                }
                </tbody>
                </table>
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
