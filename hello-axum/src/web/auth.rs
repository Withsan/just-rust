use std::fmt::Display;
use std::sync::{Arc, LazyLock};

use axum::extract::State;
use axum::http::{HeaderMap, Method, Uri};
use axum::routing::post;
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum::{Extension, Json, Router};
use axum_extra::headers::{authorization::Bearer, Authorization, HeaderMapExt};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::web::user;

use super::WebApp;
static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    let secret = "sssss";
    Keys::new(secret.as_bytes())
});

pub(crate) fn router() -> Router<Arc<WebApp>> {
    Router::new().route("/login", post(authorize))
}
async fn authorize(
    State(app): State<Arc<WebApp>>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<AuthBody>, AuthError> {
    let pool = app.db().await;
    if payload.name.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    let user = user::load_user_by_name(pool, &payload.name)
        .await
        .map_err(|_| AuthError::WrongCredentials)?;
    if user.name().ne(&payload.name) {
        return Err(AuthError::WrongCredentials);
    }
    let claims = Claims {
        sub: user.name().to_owned(),
        exp: 2000,
    };
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;
    Ok(Json(AuthBody::new(token)))
}

pub async fn authentication(
    header: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    tracing::info!("{request:?}");
    if let Some(Authorization(bearer)) = header.typed_get::<Authorization<Bearer>>() {
        if let Ok(claims) = valid_token(bearer.token()) {
            tracing::trace!("{claims:?}");
            request.extensions_mut().insert(claims);
            Ok(next.run(request).await)
        } else {
            Err(AuthError::WrongCredentials)
        }
    } else {
        Err(AuthError::MissingCredentials)
    }
}
fn valid_token(token: &str) -> Result<Claims, AuthError> {
    decode::<Claims>(&token, &KEYS.decoding, &Validation::default())
        .map(|token_data| token_data.claims)
        .map_err(|_| AuthError::InvalidToken)
}
pub async fn authorization(
    State(app): State<Arc<WebApp>>,
    Extension(claims): Extension<Claims>,
    request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    let uri = request.uri();
    let method = request.method();
    let user_name = claims.sub;
    Ok(next.run(request).await)
}
fn has_permission(name: &str, uri: &Uri, method: &Method) -> bool {
    true
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}
impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}", self.sub)
    }
}

#[derive(Debug, Serialize)]
struct AuthBody {
    access_token: String,
    token_type: String,
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}
#[derive(Debug, Deserialize)]
struct AuthPayload {
    name: String,
    password: String,
}

#[derive(Debug)]
pub(crate) enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}
