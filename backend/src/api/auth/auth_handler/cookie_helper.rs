use axum::http::{header::SET_COOKIE, HeaderMap, HeaderValue};

/// Idiomatic cookie parsing: find the "sid" pair in the Cookie header.
pub fn extract_sid(headers: &HeaderMap) -> Option<String> {
    headers
        .get(axum::http::header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|cookies| {
            cookies.split(';').find_map(|pair| {
                let (k, v) = pair.trim().split_once('=')?;
                (k.trim() == "sid").then(|| v.trim().to_string())
            })
        })
}

/// Build headers that clear the sid cookie on the client.
pub fn clear_sid_cookie_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    // Same attributes you used previously; mirror them exactly.
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_static("sid=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0"),
    );
    headers
}
