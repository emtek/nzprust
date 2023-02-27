use crate::components::{
    about::About,
    competitions::{CompetitionDetail, CompetitionList},
    nav_bar::Navbar,
    pilot_ranking::{PilotDetail, PilotList},
    ranking::RankingDetail,
};
use chrono::Utc;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum AppRoute {
    #[at("/competitions")]
    CompetitionList,
    #[at("/competition/:id")]
    CompetitionDetail { id: String },
    #[at("/pilots")]
    PilotList,
    #[at("/pilots/:pin")]
    PilotDetail { pin: String },
    #[at("/rankings")]
    RankingList,
    #[at("/ranking/:date")]
    RankingDetail { date: String },
    #[at("/about")]
    About,
    #[at("/")]
    Index,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::CompetitionList => {
            html! {
                <>
                <Navbar/>
                <CompetitionList take=1/>
                </>
            }
        }
        AppRoute::CompetitionDetail { id } => {
            html! {
                <>
                <Navbar/>
                <CompetitionDetail id={id}/>
                </>
            }
        }
        AppRoute::PilotDetail { pin } => html! { <div><Navbar/><PilotDetail pin={pin}/></div> },
        AppRoute::PilotList => {
            html! { <>
                <Navbar/>
                <PilotList take=1/>
                </>
            }
        }
        AppRoute::RankingDetail { date } => html! {
            <>
            <Navbar />
            <RankingDetail date={date}/>
            </>
        },
        AppRoute::RankingList => {
            let date = chrono::Utc::now();
            html! {
                <>
                <Navbar />
                <RankingDetail date={date.format("%Y-%m-01").to_string()}/>
                </>
            }
        }
        AppRoute::About => html! { <div><Navbar/><About/></div> },
        AppRoute::NotFound => html! { <h1>{ "404" }</h1> },
        AppRoute::Index => html! {
            <Navbar />
        },
    }
}
