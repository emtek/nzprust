use frontend::{get_authorized_data, prs_data_types::UserInfo, MultiError};
use yew::{function_component, html, Children, ContextProvider, Html, Properties};
use yew_hooks::{use_async, use_local_storage, UseLocalStorageHandle};

use crate::components::{nav_bar::Navbar, progress_bar::Progress};

#[derive(Properties, PartialEq)]
pub struct UserProfileProps {
    pub children: Children, // the field name `children` is important!
}

async fn get_profile(auth: UseLocalStorageHandle<String>) -> Result<UserInfo, MultiError> {
    let token = auth.clone();
    get_authorized_data(
        "/profile".to_string(),
        token.as_ref().unwrap_or(&"".to_string()).to_string(),
    )
    .await
}

#[function_component(UserProfile)]
pub fn user_profile(props: &UserProfileProps) -> Html {
    let token = use_local_storage::<String>("auth".to_string());
    let profile = use_async(async move { get_profile(token).await });

    match &profile.loading {
        false => match &profile.data {
            Some(data) => html! {
                <ContextProvider<UserInfo> context={data.clone()}>
                    <Navbar />
                    { for props.children.iter() }
                </ContextProvider<UserInfo>>
            },
            None => match &profile.error {
                None => {
                    profile.run();
                    html! {<></>}
                }
                _ => html! {
                    <>
                    <Navbar/>
                    { for props.children.iter() }
                    </>
                },
            },
        },
        true => html! {<Progress/>},
    }
}
