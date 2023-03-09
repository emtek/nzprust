use yew::prelude::*;

#[function_component(Progress)]
pub fn progress_indeterminate() -> Html {
    html! {
    <section class="section is-medium">
      <div class="container">
        <div class="columns is-vcentered">
          <div class="column has-text-centered">
            <progress class="progress is-info" max="100"></progress>
          </div>
        </div>
      </div>
    </section>
    }
}
