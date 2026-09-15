use std::{sync::Arc, time::{SystemTime, UNIX_EPOCH}};
use dotenvy::{dotenv, var};

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{SaltString, rand_core::OsRng}};
use axum::http::StatusCode;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Error};
use lettre::{Message, SmtpTransport, Transport, message::MultiPart, transport::smtp::{Error as SomeError, authentication::Credentials, response::Response}};
use regex::Regex;
use validator::ValidationError;

use crate::schema::{app::AppState, student::Receiver, user::Claims};

pub fn hash_password(password: &str) -> Result<String, StatusCode>{
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, StatusCode> {
    let parsed_hash = PasswordHash::new(&hash).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    
    match parsed_hash {
        Ok(ph) => Ok(Argon2::default().verify_password(password.as_bytes(), &ph).is_ok()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
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
        && has_digit.is_match(password) {
            
        true => Ok(()),
        false => Err(ValidationError::new("Need to improve your password."))
    }
}

pub fn create_token(claim: Claims, state: AppState) -> Result<String, Error> {
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize + claim.exp;

    let claim = Claims{
        id: claim.id,
        param: claim.param,
        exp: exp
    };

    encode(&Header::default(), &claim, &EncodingKey::from_secret(state.secret.as_ref()))
}

pub fn validate_token(state: Arc<AppState>, token: &str) -> bool {
    let validation = Validation::default();

    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(state.secret.as_ref()),
        &validation
    );

    token_data.is_ok()
}

pub fn send_invite_token(receiver: Receiver, token: &str) -> Result<(), SomeError> {
    dotenv().ok();

    let subject = "This is a subject";
    let name = var("SMTP_NAME").expect("Failed to load name");
    let username = var("SMTP_USERNAME").expect("Failed to load username");
    let password = var("APP_PASSWORD").expect("Failed to load password");
    let host = var("SMTP_HOST").expect("Failed to load host");
    let port: u16 = var("SMTP_PORT").expect("Failed to load port").parse().unwrap();
    
    let from = format!("{name} <{username}>");
    let to = format!("{} <{}>", receiver.name, receiver.email); 

    let message = Message::builder()
        .from(from.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .multipart(MultiPart::alternative_plain_html(
            String::from("Hello, there!"),
            String::from(token),
    )); //TODO: format the token so we send it as a link for user registration

    let sender: SmtpTransport = SmtpTransport::starttls_relay(&host)?
    .credentials(Credentials::new(
        username.to_owned(),
        password.to_owned(),
    ))
    .port(port)
    .build();

    // Send the email via remote relay
    let response: Result<Response, SomeError> = sender.send(&message.unwrap());

    match response {
        Ok(_) => Ok(()),
        Err(err) => Err(err)
    }
}
