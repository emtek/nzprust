use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use axum::body::Body;
use axum::http::header::AUTHORIZATION;
use frontend::prs_data_types::UserInfo;
use google_signin::CachedCerts;

pub async fn google_auth(
    state: State<(Vec<String>, CachedCerts)>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let (admin_users, mut cached_certs) = state.0;
    let _ = cached_certs.refresh_if_needed().await.ok().ok_or(false);
    let mut client = google_signin::Client::new();
    client.audiences.push(
        "478528255102-n9lfj6vqg2rsv0drdo7s50mo99pl4ugd.apps.googleusercontent.com".to_string(),
    ); // required
    let token = request
        .headers().get(&AUTHORIZATION);
    match token {
        None => Err(StatusCode::UNAUTHORIZED),
        Some(token) => {
            let token_str = &token.to_str().unwrap().replace("Bearer ", "");
            let id_info = client
                .verify(token_str, &cached_certs)
                .await;
            match id_info{
                Ok(id_info) =>{let email = id_info.email.unwrap_or_default().clone();
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
                    }},
                Err(error) => {
                    let error_message = error.to_string();
                    Err(StatusCode::UNAUTHORIZED)
                }
            }

        }
    }

}
