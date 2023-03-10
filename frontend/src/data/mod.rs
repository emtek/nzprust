pub mod highcloud_data_types;
pub mod prs_data_types;

fn get_base_url() -> String {
    if let Some(window) = web_sys::window() {
        format!("{}{}", window.origin(), "/api")
    } else {
        format!("http://localhost:8000/api") //fallback
    }
}

pub async fn get_data<T>(path: String) -> Result<T, MultiError>
where
    T: serde::de::DeserializeOwned,
{
    get_data_external::<T>(format!("{}{}", get_base_url(), path)).await
}

pub async fn get_data_external<T>(path: String) -> Result<T, MultiError>
where
    T: serde::de::DeserializeOwned,
{
    let response = reqwest::get(path).await;
    match response {
        Err(_) => Err(MultiError::RequestError),
        Ok(response) => match response.text().await {
            Err(_) => Err(MultiError::RequestError),
            Ok(text) => match serde_json::from_str::<T>(&text) {
                Err(_) => Err(MultiError::DeserializeError),
                Ok(result) => Ok(result),
            },
        },
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MultiError {
    RequestError,
    DeserializeError,
    // etc.
}
