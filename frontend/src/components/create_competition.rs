use std::rc::Rc;

use crate::{data::prs_data_types::Competition, data::*, routes::AppRoute};

use validator::Validate;
use yew::prelude::*;
use yew_router::prelude::Link;
use yewdux::prelude::*;
use yewdux_input::InputDispatch;

fn is_valid(field: &str, state: &Rc<Competition>) -> Option<String> {
    match state.validate() {
        Err(error) => {
            if error
                .field_errors()
                .iter()
                .any(|(f, _)| f.clone().cmp(&field.clone()).is_eq())
            {
                Some("is-danger".to_string())
            } else {
                None
            }
        }
        Ok(_) => None,
    }
}

fn validation_message(field: &str, state: &Rc<Competition>) -> Option<String> {
    match state.validate() {
        Err(error) => Some(
            error
                .field_errors()
                .iter()
                .filter(|(f, _)| f.clone().cmp(&field.clone()).is_eq())
                .map(|(_, v)| {
                    format!(
                        "{:?}",
                        v.into_iter()
                            .filter(|f| f.message.is_some())
                            .map(|f| f.message.clone().unwrap().to_string())
                            .collect::<Vec<String>>()
                            .join(",")
                    )
                })
                .collect::<Vec<String>>()
                .join(","),
        ),
        Ok(_) => None,
    }
}

async fn get_highcloud_comp(url_string: &String) -> Result<Competition, MultiError> {
    get_data(format!("/competition/fromhc/{}", url_string)).await
}

#[function_component(CompetitionCreate)]
pub fn competition_create() -> Html {
    let (state, dispatch) = use_store::<Competition>();
    let onclick = dispatch.reduce_mut_future_callback(|state| {
        Box::pin(async move {
            if let Ok(comp) = get_highcloud_comp(&state.name).await {
                web_sys::console::log_1(&comp.name.clone().into());
                state.name = comp.name;
                state.location = comp.location;
                state.comp_date = comp.comp_date;
                state.num_tasks = comp.num_tasks;
                state.placings = comp.placings;
            }
            ()
        })
    });

    fn exchange_rate_visible(state: &Rc<Competition>) -> Option<String> {
        if state.overseas {
            None
        } else {
            Some("is-hidden".to_string())
        }
    }

    html! {
    <>
    <section class="hero is-info">
        <div class="hero-body">
            <p class="title">
            { "Create competition" }
            </p>
        </div>
    </section>
    <section class="section">

      <div class="field">
        <label class="label">{"Name"}</label>
        <div class="control">
          <input value={dispatch.get().name.clone()} oninput={dispatch.input_mut(|state, text| state.name = text)} class={classes!("input",is_valid("name", &state))} type="text" placeholder="Name"/>
        </div>
        <p class="help is-danger">{validation_message("name", &state)}</p>
      </div>

      <div class="field">
        <label class="label">{"Location"}</label>
        <div class="control">
          <input value={dispatch.get().location.clone()} oninput={dispatch.input_mut(|state, text| state.location = text)} class={classes!("input",is_valid("location", &state))} type="text" placeholder="Location"/>
        </div>
        <p class="help is-danger">{validation_message("location", &state)}</p>
      </div>

      <div class="field">
        <label class="label">{"Date YYYY-MM-DD"}</label>
        <div class="control">
          <input type="date" value={dispatch.get().comp_date.clone()} oninput={dispatch.input_mut(|state, text| state.comp_date = text)} class={classes!("input",is_valid("comp_date", &state))} type="text" placeholder="Date"/>
        </div>
        <p class="help is-danger">{validation_message("comp_date", &state)}</p>
      </div>

      <div class="field">
        <label class="label">{"Number of tasks"}</label>
        <div class="control">
          <input type="number" value={dispatch.get().num_tasks.to_string()} oninput={dispatch.input_mut(|state, text| state.num_tasks = text)} class={classes!("input",is_valid("num_tasks", &state))} type="text" placeholder="Number of tasks"/>
        </div>
        <p class="help is-danger">{validation_message("num_tasks",&state)}</p>
      </div>

      <div class="field">
        <input id="switchRoundedInfo" type="checkbox" onclick={dispatch.reduce_mut_callback(|state| state.overseas = !state.overseas)}  name="switchRoundedInfo" class="switch is-rounded is-info" checked={dispatch.get().overseas}/>
        <label for="switchRoundedInfo">{"Overseas"}</label>
      </div>

      <div class={classes!("field",exchange_rate_visible(&state))}>
        <label class="label">{"Exchange rate"}</label>
        <div class="control">
          <input type="number" oninput={dispatch.input_mut(|state, text| state.exchange_rate = text)} class={classes!("input",is_valid("exchange_rate", &state))} type="text" placeholder="Number of tasks"/>
        </div>
        <p class="help is-danger">{validation_message("exchange_rate",&state)}</p>
      </div>

      <div class="control">
        <table class="table is-fullwidth">
            <thead>
                <tr>
                <th>{"Rank"}</th>
                <th>{"Pilot"}</th>
                </tr>
            </thead>
            <tbody>
            {
                dispatch.get().placings.iter().map(|placing|
                    html!{
                    <tr>
                        <td>{&placing.place}</td>
                        <td><Link<AppRoute> to={AppRoute::PilotDetail {pin: placing.pilot.pin.clone()}}>
                            {format!("{}", &placing.pilot.first_name)}
                        </Link<AppRoute>></td>
                    </tr>
                }).collect::<Html>()
            }
            </tbody>
          </table>
      </div>

      <div class="field is-grouped">
        <div class="control">
          <button class="button is-link" {onclick}>{"Submit"}</button>
        </div>
        <div class="control">
          <Link<AppRoute> to={AppRoute::CompetitionList} >
            <button class="button is-link is-light" >{"Cancel"}</button>
          </Link<AppRoute>>
        </div>
      </div>
    </section>
    </>
    }
}
