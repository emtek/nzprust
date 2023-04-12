use std::rc::Rc;

use crate::{data::prs_data_types::Competition, data::*, routes::AppRoute};

use validator::Validate;
use yew::prelude::*;
use yew_router::prelude::Link;
use yewdux::prelude::*;
use yewdux_input::InputDispatch;

fn is_valid(field: &str, state: &Rc<Competition>) -> Option<String> {
    match validation_message(field, state) {
        Some(_) => Some("is-danger".to_string()),
        None => None,
    }
}

fn validation_message(field: &str, state: &Rc<Competition>) -> Option<String> {
    match state.validate() {
        Err(error) => {
            let field_messages = error
                .field_errors()
                .iter()
                .filter(|(f, _)| f.clone().cmp(&field.clone()).is_eq())
                .map(|(_, v)| {
                    format!(
                        "{}",
                        v.into_iter()
                            .filter(|f| f.message.is_some())
                            .map(|f| f.message.clone().unwrap().to_string())
                            .collect::<Vec<String>>()
                            .join(",")
                    )
                })
                .collect::<Vec<String>>();
            let overall_messages = error
                .field_errors()
                .iter()
                .filter(|(f, _)| f.clone().contains(&"__all__"))
                .map(|(_, v)| {
                    format!(
                        "{}",
                        v.into_iter()
                            .filter(|f| f.code.contains(field))
                            .map(|f| f.message.clone().unwrap().to_string())
                            .collect::<Vec<String>>()
                            .join("")
                    )
                })
                .filter(|s| s.len() > 0)
                .collect::<Vec<String>>();
            let combined = [field_messages, overall_messages].concat();
            if combined.len() == 0 {
                None
            } else {
                Some(combined.join(","))
            }
        }
        Ok(_) => None,
    }
}

async fn get_highcloud_comp(url_string: &String) -> Result<Competition, MultiError> {
    get_data(format!("/competition/fromhc/{}", url_string)).await
}

async fn get_fai_comp(url_string: &String) -> Result<Competition, MultiError> {
    get_data(format!("/competition/fromfai/{}", url_string)).await
}

#[function_component(CompetitionCreate)]
pub fn competition_create() -> Html {
    let (state, dispatch) = use_store::<Competition>();
    let from_fai = dispatch.reduce_mut_future_callback(|state| {
        Box::pin(async move {
            if let Ok(comp) = get_fai_comp(&state.name).await {
                web_sys::console::log_1(&comp.name.clone().into());
                state.name = comp.name;
                state.location = comp.location;
                state.comp_date = comp.comp_date;
                state.num_tasks = comp.num_tasks;
                state.overseas = comp.overseas;
                state.placings = comp.placings;
            }
            ()
        })
    });

    let from_hc = dispatch.reduce_mut_future_callback(|state| {
        Box::pin(async move {
            if let Ok(comp) = get_highcloud_comp(&state.name).await {
                web_sys::console::log_1(&comp.name.clone().into());
                state.name = comp.name;
                state.location = comp.location;
                state.comp_date = comp.comp_date;
                state.num_tasks = comp.num_tasks;
                state.overseas = comp.overseas;
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

    fn submit_disabled(state: &Rc<Competition>) -> bool {
        match state.validate() {
            Err(_) => true,
            Ok(_) => false,
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
    <div class="field is-grouped">
      <div class="control">
        <button class="button is-link" onclick={from_hc}>{"From highcloud"}</button>
      </div>
      <div class="control">
        <button class="button is-link" onclick={from_fai}>{"From fai"}</button>
      </div>
    </div>
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
        <label class="label">{"Start Date"}</label>
        <div class="control">
          <input type="date" value={dispatch.get().comp_date.clone()} oninput={dispatch.input_mut(|state, text| state.comp_date = text)} class={classes!("input",is_valid("comp_date", &state))} type="text" placeholder="YYYY-MM-DD"/>
        </div>
        <p class="help is-danger">{validation_message("comp_date", &state)}</p>
      </div>

      <div class="field">
        <label class="label">{"Number of tasks"}</label>
        <div class="control">
          <input value={dispatch.get().num_tasks.to_string()}  type="number"  oninput={dispatch.input_mut(|state, text| state.num_tasks = text)} class={classes!("input",is_valid("num_tasks", &state))} type="text" placeholder="Number of tasks"/>
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
          <input type="number" oninput={dispatch.input_mut(|state, text| state.exchange_rate = text)} class={classes!("input",is_valid("exchange_rate", &state))} type="text" placeholder="Exchange rate"/>
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
          <button class="button is-link" disabled={submit_disabled(&state)}>{"Submit"}</button>
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_competition_returns_errors() {
        let competition = Rc::new(Competition {
            name: "Morethan3".to_string(),
            location: "location".to_string(),
            comp_date: "2022-12-".to_string(),
            num_tasks: 4,
            overseas: true,
            ..Default::default()
        });
        let exchange_rate_message = validation_message(&"exchange_rate", &competition);
        let comp_date_message = validation_message(&"comp_date", &competition);
        assert_eq!(exchange_rate_message.is_some(), true);
        assert_eq!(comp_date_message.is_some(), true);
    }
}
