use axum::{http::StatusCode, response::Redirect};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use rand::{thread_rng, Rng};

/// Creates a new cookie with a new random value between 1 and 100, then it
/// redirects the client to "/".
pub async fn new(jar: CookieJar) -> (CookieJar, Redirect) {
    let n = thread_rng().gen_range(1..=100) as u32;
    // Of course, you probably should not do this in a real app.
    let cookie = Cookie::new("secret", n.to_string());
    (jar.add(cookie), Redirect::to("/"))
}

/// WARNING: This handler will be deleted.
pub async fn deleteme(jar: CookieJar) -> Result<String, StatusCode> {
    if let Some(secret) = jar.get("secret") {
        Ok(secret.value().into())
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

pub async fn index() -> &'static str {
    "Hello world"
}
