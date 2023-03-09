use crate::components::{
    about::About,
    competitions::{CompetitionDetail, CompetitionList},
    create_competition::CompetitionCreate,
    nav_bar::Navbar,
    not_found::NotFound,
    pilots::{PilotDetail, PilotList},
    ranking::RankingDetail,
};

use chrono::NaiveDate;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum AppRoute {
    #[at("/competitions")]
    CompetitionList,
    #[at("/competition/:id")]
    CompetitionDetail { id: String },
    #[at("/competition/new")]
    CompetitionNew,
    #[at("/pilots")]
    PilotList,
    #[at("/pilots/:pin")]
    PilotDetail { pin: String },
    #[at("/rankings")]
    RankingList,
    #[at("/ranking/:date")]
    RankingDetail { date: NaiveDate },
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
        AppRoute::Index => {
            let date = chrono::Utc::now();
            html! {
            <>
                <Navbar />
                <RankingDetail date={date.date_naive()}/>
            </>
            }
        }
        AppRoute::CompetitionList => {
            html! {
                <>
                <Navbar/>
                <CompetitionList/>
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
        AppRoute::CompetitionNew => {
            html! {
                <>
                <Navbar/>
                <CompetitionCreate/>
                </>
            }
        }
        AppRoute::PilotDetail { pin } => html! { <div><Navbar/><PilotDetail pin={pin}/></div> },
        AppRoute::PilotList => {
            html! { <>
                <Navbar/>
                <PilotList/>
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
                <RankingDetail date={date.date_naive()}/>
                </>
            }
        }
        AppRoute::About => html! { <div><Navbar/><About/></div> },
        AppRoute::NotFound => html! { <><Navbar/><NotFound/></> },
    }
}
