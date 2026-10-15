use dotenvy::{dotenv, var};
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{self, Error, SaltString, rand_core::OsRng},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use lettre::{
    Message, SmtpTransport, Transport,
    message::MultiPart,
    transport::smtp::{Error as SomeError, authentication::Credentials, response::Response},
};
use regex::Regex;
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
