use reqwest::StatusCode;

pub mod prs_data_types;

pub async fn get_data<T>(path: String) -> Result<T, MultiError>
where
    T: serde::de::DeserializeOwned,
{
    get_data_external::<T>(format!("{}{}", get_base_url(), path), None).await
}

pub async fn get_authorized_data<T>(path: String, token: String) -> Result<T, MultiError>
where
    T: serde::de::DeserializeOwned,
{
    get_data_external::<T>(format!("{}{}", get_base_url(), path), Some(token)).await
}

pub async fn get_data_external<T>(path: String, token: Option<String>) -> Result<T, MultiError>
where
    T: serde::de::DeserializeOwned,
{
    let response = reqwest::ClientBuilder::new()
        .build()
        .ok()
        .ok_or(MultiError::RequestError)?
        .get(path)
        .bearer_auth(&token.unwrap_or_default().as_str())
        .send()
        .await;
    match response {
        Err(_) => Err(MultiError::RequestError),
        Ok(response) => match response.status() {
            StatusCode::UNAUTHORIZED => Err(MultiError::AuthorizationError),
            _ => match response.text().await {
                Err(_) => Err(MultiError::DeserializeError),
                Ok(text) => match serde_json::from_str::<T>(&text) {
                    Err(_) => Err(MultiError::DeserializeError),
                    Ok(result) => Ok(result),
                },
            },
        },
    }
}

fn get_base_url() -> String {
    if let Some(window) = web_sys::window() {
        match window.origin().contains("127") {
            true => format!("http://127.0.0.1:8080/api"), //fallback
            false => format!("{}{}", window.origin(), "/api"),
        }
    } else {
        format!("http://127.0.0.1:8080/api") //fallback
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MultiError {
    RequestError,
    DeserializeError,
    AuthorizationError, // etc.
}
