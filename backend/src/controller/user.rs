use axum::{
    extract::{Extension, TypedHeader},
    response::{Headers, IntoResponse},
    routing::post,
    Json, Router,
};
use headers::Cookie;
use serde_json::{json, Value};

use crate::controller::models::{LoginExtract, SignUpExtract};
use crate::domain::entity::user::{
    AccessToken, LoginError, RefreshToken, RefreshTokenError, RefreshTokenExtract, SignUpError,
};
use crate::domain::service::user::UserService;
use crate::settings::Settings;

pub fn user_app() -> Router {
    Router::new()
        .route("/login-session", post(make_login_session))
        .route("/login", post(login))
        .route("/signup", post(sign_up))
        .route("/token", post(refresh_tokens))
}

async fn make_login_session(user_service: UserService) -> Result<Json<Value>, LoginError> {
    user_service.make_login_session().await.map(|session| {
        Json(json!({
            "session_id": session.session_id,
            "nonce": session.nonce,
            "code_challenge": session.code_challenge.as_str().to_string(),
        }))
    })
}

fn response_from_tokens(
    cookie_name: &str,
    refresh_token: RefreshToken,
    access_token: AccessToken,
) -> impl IntoResponse {
    (
        Headers(vec![(
            "Set-Cookie",
            refresh_token.into_cookie_value(cookie_name, "/token"),
        )]),
        access_token.0,
    )
}

async fn login(
    user_service: UserService,
    Json(payload): Json<LoginExtract>,
    Extension(settings): Extension<Settings>,
) -> Result<impl IntoResponse, LoginError> {
    user_service
        .login(payload.session_id, payload.code)
        .await
        .map(|ts| response_from_tokens(&settings.refresh_token_cookie_name, ts.0, ts.1))
}

async fn sign_up(
    user_service: UserService,
    Json(payload): Json<SignUpExtract>,
    Extension(settings): Extension<Settings>,
) -> Result<impl IntoResponse, SignUpError> {
    user_service
        .sign_up(payload.code, payload.user)
        .await
        .map(|ts| response_from_tokens(&settings.refresh_token_cookie_name, ts.0, ts.1))
}

async fn refresh_tokens(
    user_service: UserService,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Extension(settings): Extension<Settings>,
) -> Result<impl IntoResponse, RefreshTokenError> {
    let refresh_token_value = cookie
        .get(&settings.refresh_token_cookie_name)
        .ok_or(RefreshTokenError::InvalidRefreshToken)?;

    user_service
        .refresh_tokens(RefreshTokenExtract(refresh_token_value.to_string()))
        .await
        .map(|ts| response_from_tokens(&settings.refresh_token_cookie_name, ts.0, ts.1))
}
