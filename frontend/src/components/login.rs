use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
    <section class="section">
              <div class="buttons">
              <div id="g_id_onload"
              data-client_id="478528255102-n9lfj6vqg2rsv0drdo7s50mo99pl4ugd.apps.googleusercontent.com"
              data-context="signin"
              data-ux_mode="popup"
              data-callback="receive_token"
              data-auto_prompt="false">
         </div>

         <div class="g_id_signin"
              data-type="standard"
              data-shape="rectangular"
              data-theme="filled_blue"
              data-text="signin_with"
              data-size="large"
              data-logo_alignment="left">
         </div>

          </div>
          <script src="https://accounts.google.com/gsi/client" async=true defer=true></script>
          </section>
    }
}
