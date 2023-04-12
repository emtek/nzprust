use crate::components::{
    about::About,
    competitions::{CompetitionDetail, CompetitionList},
    create_competition::CompetitionCreate,
    login::Login,
    nav_bar::Navbar,
    not_found::NotFound,
    pilots::{PilotDetail, PilotList},
    ranking::RankingDetail,
    user_profile::UserProfile,
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
    #[at("/login")]
    Login,
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
                <UserProfile>
                    <CompetitionList/>
                </UserProfile>
            }
        }
        AppRoute::CompetitionDetail { id } => {
            html! {
                <UserProfile>
                    <CompetitionDetail id={id}/>
                </UserProfile>
            }
        }
        AppRoute::CompetitionNew => {
            html! {
                <UserProfile>
                    <CompetitionCreate/>
                </UserProfile>
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
        AppRoute::Login => html! { <div><Navbar/><Login/></div> },
        AppRoute::NotFound => html! { <><Navbar/><NotFound/></> },
    }
}
