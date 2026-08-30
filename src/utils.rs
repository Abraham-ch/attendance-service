use std::{sync::Arc, time::{SystemTime, UNIX_EPOCH}};

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{SaltString, rand_core::OsRng}};
use axum::{Json, http::StatusCode};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use regex::Regex;
use validator::ValidationError;

use crate::schema::{app::AppState, user::{AuthResponse, Claims, User}};

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

pub fn create_token(user: User, state: AppState) -> Result<(StatusCode, Json<AuthResponse>), (StatusCode, String)> {
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize + 86400;

    let claim = Claims{
        sub: user.id.to_string(),
        role: user.role.clone(),
        exp: exp
    };

    let token = encode(&Header::default(), &claim, &EncodingKey::from_secret(state.secret.as_ref()));

    match token {
        Ok(result) => Ok((StatusCode::OK, Json(AuthResponse {user, token: result}))),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Token couldn't be generated.".to_string()))
    }
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
