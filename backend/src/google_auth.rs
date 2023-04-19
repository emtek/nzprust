use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use frontend::prs_data_types::UserInfo;
use google_signin::CachedCerts;

pub async fn google_auth<T>(
    state: State<(Vec<String>, CachedCerts)>,
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, StatusCode> {
    let (admin_users, cached_certs) = state.0;
    let mut client = google_signin::Client::new();
    client.audiences.push(
        "478528255102-n9lfj6vqg2rsv0drdo7s50mo99pl4ugd.apps.googleusercontent.com".to_string(),
    ); // required
    let token = request
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or(StatusCode::UNAUTHORIZED)?
        .token()
        .to_owned();
    let id_info = client
        .verify(&token, &cached_certs)
        .await
        .ok()
        .ok_or(StatusCode::UNAUTHORIZED)?;
    let email = id_info.email.unwrap_or_default().clone();
    let user = admin_users.iter().find(|u| u.eq_ignore_ascii_case(&email));
    match user {
        Some(_) => {
            request.extensions_mut().insert(UserInfo {
                email: email.clone(),
                name: id_info.name.unwrap_or_default().clone(),
                picture: id_info.picture.unwrap_or_default().clone(),
            });
            Ok(next.run(request).await)
        }
        None => Err(StatusCode::UNAUTHORIZED),
    }
}
