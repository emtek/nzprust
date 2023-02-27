use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::*;
#[function_component(Navbar)]
pub fn nav_bar() -> Html {
    let state = use_state(move || NavState {
        hamburger_visible: false,
    });
    let visible = match state.hamburger_visible {
        true => Some("is-active"),
        false => None,
    };
    let toggle_hamburger = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(NavState {
                hamburger_visible: !state.hamburger_visible,
            })
        })
    };

    html! {
    <nav class="navbar" role="navigation" aria-label="main navigation">
      <div class="navbar-brand">
        <a class="navbar-item" href="https://nzhgpa.org.nz">
          <img src="https://nzhgpa.org.nz/wp-content/uploads/2022/03/logo_light.png"/>
        </a>

        <a onclick={toggle_hamburger} role="button" class={classes!("navbar-burger",visible)} aria-label="menu" aria-expanded="false" data-target="navbarBasicExample">
          <span aria-hidden="true"></span>
          <span aria-hidden="true"></span>
          <span aria-hidden="true"></span>
        </a>
      </div>

      <div id="navbarBasicExample" class={classes!("navbar-menu",visible)}>
        <div class="navbar-start">
          <a class="navbar-item">
            <Link<AppRoute> to={AppRoute::RankingList}>{ "Rankings" }</Link<AppRoute>>
          </a>

          <a class="navbar-item">
            <Link<AppRoute> to={AppRoute::PilotList}>{ "Pilots" }</Link<AppRoute>>
          </a>

          <a class="navbar-item">
           <Link<AppRoute> to={AppRoute::CompetitionList}>{"Competitions"}</Link<AppRoute>>

          </a>

          <a class="navbar-item">
            <Link<AppRoute> to={AppRoute::About}>{ "About" }</Link<AppRoute>>
          </a>

        </div>
      </div>
    </nav>
    }
}

struct NavState {
    hamburger_visible: bool,
}
