use dotenvy::{dotenv, var};
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::Context;
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{self, Error, SaltString, rand_core::OsRng},
};
use chrono::{NaiveDate, NaiveTime};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use lettre::{
    Message, SmtpTransport, Transport,
    message::MultiPart,
    transport::smtp::{Error as SomeError, authentication::Credentials, response::Response},
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::ValidationError;

use crate::schema::{app::AppState, student::Receiver, user::Claims};

pub fn hash_password(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    match argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| Error::from(e))
    {
        Ok(hash) => Ok(hash),
        Err(e) => Err(e),
    }
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, Error> {
    let parsed_hash = PasswordHash::new(&hash)?;

    match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(password_hash::Error::Password) => Ok(false),
        Err(e) => Err(e),
    }
}

pub fn valid_password(password: &str) -> Result<(), ValidationError> {
    let has_lowercase = Regex::new(r"[a-z]").unwrap();
    let has_uppercase = Regex::new(r"[A-Z]").unwrap();
    let has_special_char = Regex::new(r"[^a-zA-Z0-9]").unwrap();
    let has_digit = Regex::new(r"[0-9]").unwrap();

    match has_lowercase.is_match(password)
        && has_uppercase.is_match(password)
        && has_special_char.is_match(password)
        && has_digit.is_match(password)
    {
        true => Ok(()),
        false => Err(ValidationError::new("Need to improve your password.")),
    }
}

pub fn create_token(claim: Claims, state: AppState) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        + claim.exp;

    let claim = Claims {
        id: claim.id,
        param: claim.param,
        exp: exp,
    };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(state.secret.as_ref()),
    )
}

//just validate if hasn't expired and have the secret key
pub fn validate_token(state: Arc<AppState>, token: &str) -> bool {
    let validation = Validation::default();

    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(state.secret.as_ref()),
        &validation,
    );

    token_data.is_ok()
}

pub fn decode_token(state: AppState, token: &str) -> Option<Claims> {
    let validation = Validation::default();

    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(state.secret.as_ref()),
        &validation,
    )
    .ok()
    .map(|token_data| token_data.claims)
}

#[derive(Serialize, Deserialize)]
struct DailyClaims {
    date: String,
    /// where the admin showing the QR was, check-in only works near it
    lat: f64,
    lng: f64,
    exp: usize,
}

// A key of its own: a daily code must never pass auth_middleware as a login token
fn daily_key(secret: &str) -> String {
    format!("{secret}:daily-check-in")
}

/// The code inside the day's check-in QR, a JWT that expires when the (UTC) day ends.
/// It carries the spot it was shown at, so a photo of it sent to someone at home is useless:
/// check_in only accepts students within check_in_radius_m() of that spot.
// their GPS. Pair it with a rotating code (date -> 30s window) if that starts happening
pub fn daily_code(
    secret: &str,
    date: NaiveDate,
    (lat, lng): (f64, f64),
) -> Result<String, jsonwebtoken::errors::Error> {
    let end_of_day = date
        .succ_opt()
        .unwrap_or(date)
        .and_time(NaiveTime::MIN)
        .and_utc();
    let claims = DailyClaims {
        date: date.to_string(),
        lat,
        lng,
        exp: end_of_day.timestamp() as usize,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(daily_key(secret).as_ref()),
    )
}

/// Where today's code was shown, None when it is forged, expired or from another day
pub fn daily_code_origin(secret: &str, code: &str, today: NaiveDate) -> Option<(f64, f64)> {
    let claims = decode::<DailyClaims>(
        code,
        &DecodingKey::from_secret(daily_key(secret).as_ref()),
        &Validation::default(),
    )
    .ok()?
    .claims;
    (claims.date == today.to_string()).then_some((claims.lat, claims.lng))
}

/// Rejects NaN too, so a distance computed from it can't slip through a comparison
pub fn valid_location((lat, lng): (f64, f64)) -> bool {
    (-90.0..=90.0).contains(&lat) && (-180.0..=180.0).contains(&lng)
}

/// Metres between two points (haversine), plenty for "is the student in the room"
pub fn distance_m((lat1, lng1): (f64, f64), (lat2, lng2): (f64, f64)) -> f64 {
    const EARTH_RADIUS_M: f64 = 6_371_000.0;
    let d_lat = (lat2 - lat1).to_radians();
    let d_lng = (lng2 - lng1).to_radians();
    let a = (d_lat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (d_lng / 2.0).sin().powi(2);
    2.0 * EARTH_RADIUS_M * a.sqrt().asin()
}

/// How far from the QR a student may check in. Phone GPS indoors is often off by 20-100 m,
/// so tune CHECK_IN_RADIUS_METERS in .env to the building instead of changing the code
pub fn check_in_radius_m() -> f64 {
    var("CHECK_IN_RADIUS_METERS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(150.0)
}

/// Uploads an avatar to the Supabase Storage bucket and returns its public URL.
/// Needs SUPABASE_URL, SUPABASE_SERVICE_ROLE_KEY and SUPABASE_AVATAR_BUCKET (a public bucket).
pub async fn store_avatar(
    name: &str,
    content_type: &str,
    image: axum::body::Bytes,
) -> anyhow::Result<String> {
    let base = var("SUPABASE_URL").context("SUPABASE_URL is not set")?;
    let key = var("SUPABASE_SERVICE_ROLE_KEY").context("SUPABASE_SERVICE_ROLE_KEY is not set")?;
    let bucket = var("SUPABASE_AVATAR_BUCKET").context("SUPABASE_AVATAR_BUCKET is not set")?;
    let base = base.trim_end_matches('/');

    let response = reqwest::Client::new()
        .post(format!("{base}/storage/v1/object/{bucket}/{name}"))
        .bearer_auth(&key)
        .header("apikey", &key)
        .header(reqwest::header::CONTENT_TYPE, content_type)
        .body(image)
        .send()
        .await?;
    if !response.status().is_success() {
        anyhow::bail!(
            "Supabase Storage answered {}: {}",
            response.status(),
            response.text().await.unwrap_or_default()
        );
    }
    Ok(format!("{base}/storage/v1/object/public/{bucket}/{name}"))
}

fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// The register page link inside the invite email
pub fn invite_url(web_url: &str, token: &str) -> String {
    format!(
        "{}/auth/register?token={token}",
        web_url.trim_end_matches('/')
    )
}

pub fn invite_html(name: &str, url: &str) -> String {
    include_str!("../templates/invite.html")
        .replace("{{name}}", &escape_html(name))
        .replace("{{url}}", &escape_html(url))
}

pub fn send_invite_token(receiver: Receiver, token: &str) -> Result<(), SomeError> {
    dotenv().ok();

    let subject = "Activate your attendance account";
    // the web app's origin, already configured for CORS
    let web_url = var("ALLOWED_ORIGIN").expect("Failed to load web url");
    let url = invite_url(&web_url, token);
    let name = var("SMTP_NAME").expect("Failed to load name");
    let username = var("SMTP_USERNAME").expect("Failed to load username");
    let password = var("APP_PASSWORD").expect("Failed to load password");
    let host = var("SMTP_HOST").expect("Failed to load host");
    let port: u16 = var("SMTP_PORT")
        .expect("Failed to load port")
        .parse()
        .unwrap();

    let from = format!("{name} <{username}>");
    let to = format!("{} <{}>", receiver.name, receiver.email);

    let message = Message::builder()
        .from(from.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .multipart(MultiPart::alternative_plain_html(
            format!(
                "Hi {},
                Your school registered you in the attendance system.
                Activate your account here (the link works once and expires in 3 days):
                {url}
                ",
                receiver.name
            ),
            invite_html(&receiver.name, &url),
        ));

    let sender: SmtpTransport = SmtpTransport::starttls_relay(&host)?
        .credentials(Credentials::new(username.to_owned(), password.to_owned()))
        .port(port)
        .build();

    // Send the email via remote relay
    let response: Result<Response, SomeError> = sender.send(&message.unwrap());

    match response {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CLASSROOM: (f64, f64) = (-12.0464, -77.0428);

    #[tokio::test]
    async fn daily_code_only_checks_in_on_its_own_day() {
        let today = chrono::Utc::now().date_naive();
        let code = daily_code("secret", today, CLASSROOM).unwrap();

        assert_eq!(
            code,
            daily_code("secret", today, CLASSROOM).unwrap(),
            "same QR all day"
        );
        assert_eq!(daily_code_origin("secret", &code, today), Some(CLASSROOM));
        assert_eq!(
            daily_code_origin("other", &code, today),
            None,
            "signed with our secret"
        );
        assert_eq!(
            daily_code_origin("secret", &code, today.succ_opt().unwrap()),
            None
        );

        let yesterday = daily_code("secret", today.pred_opt().unwrap(), CLASSROOM).unwrap();
        assert_eq!(
            daily_code_origin("secret", &yesterday, today),
            None,
            "expired"
        );

        // a daily code is not a login token
        let state = AppState {
            pool: sqlx::PgPool::connect_lazy("postgres://localhost/unused").unwrap(),
            secret: "secret".into(),
            tx: tokio::sync::broadcast::channel(1).0,
        };
        assert!(decode_token(state, &code).is_none());
    }

    #[test]
    fn distance_and_location_checks() {
        assert!(distance_m(CLASSROOM, CLASSROOM) < 0.001);
        // 0.001 degrees of latitude is ~111 m
        let next_block = (CLASSROOM.0 + 0.001, CLASSROOM.1);
        assert!((distance_m(CLASSROOM, next_block) - 111.2).abs() < 1.0);
        // Lima to Cusco, ~570 km
        let cusco = (-13.5320, -71.9675);
        assert!((distance_m(CLASSROOM, cusco) / 1000.0 - 570.0).abs() < 15.0);

        assert!(valid_location(CLASSROOM));
        assert!(!valid_location((91.0, 0.0)));
        assert!(!valid_location((f64::NAN, 0.0)));
    }

    #[test]
    fn invite_email_links_to_register_page_and_escapes_name() {
        let url = invite_url("http://localhost:5173/", "abc.def-ghi");
        assert_eq!(url, "http://localhost:5173/auth/register?token=abc.def-ghi");

        let html = invite_html("<b>Ana</b>", &url);
        assert!(html.contains("&lt;b&gt;Ana&lt;/b&gt;"));
        assert!(html.contains(r#"href="http://localhost:5173/auth/register?token=abc.def-ghi""#));
        assert!(!html.contains("{{"));
    }
}
