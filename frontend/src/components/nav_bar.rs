use frontend::prs_data_types::UserInfo;
use yew::prelude::*;
use yew_hooks::use_local_storage;
use yew_router::prelude::*;

use crate::routes::*;
#[function_component(Navbar)]
pub fn nav_bar() -> Html {
    let token = use_local_storage::<String>("auth".to_string());
    let state = use_state(move || NavState {
        hamburger_visible: false,
    });
    let force_update = use_force_update();
    let user = use_context::<UserInfo>();
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

    let logout = {
        let state = token.clone();
        let update_handle = force_update.clone();
        Callback::from(move |_| {
            state.delete();
            update_handle.force_update();
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
            <Link<AppRoute> to={AppRoute::About}>{"About"} </Link<AppRoute>>
          </a>

        </div>
        <div class="navbar-end">
        <div class="navbar-item">
        <div class="buttons">
          { if let Some(_) = user {
            html!{
              <a onclick={logout} class="button is-light is-rounded">
                // <img src={user.picture}/>
                <span>{"   Log out"}</span>
              </a>

            }
          }else{
              html!{<></>}
            }
          }
          </div>
          </div>
        </div>
      </div>

    </nav>

    }
}

struct NavState {
    hamburger_visible: bool,
}
