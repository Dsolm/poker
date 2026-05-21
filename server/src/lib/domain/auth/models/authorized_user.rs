use argon2::{
    Argon2, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display},
    str::FromStr,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub(crate) sub: String, // subject (user id)
    exp: usize,             // expiration timestamp
}

#[derive(Debug, Clone)]
pub struct UserName(pub(crate) String);

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Email(String);
pub struct InvalidEmail;

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub password_hash: PasswordHash,
    pub username: UserName,
    pub email: Email,
}

pub struct CreateUserRequest {
    pub id: String,
    pub password_hash: PasswordHash,
    pub username: UserName,
    pub email: Email,
}

pub struct Token(pub String);

// let secret = std::env::var("JWT_SECRET").expect("missing secret");
pub fn create_token(user_id: String, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::days(30))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn verify_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

#[derive(Debug, Clone)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(password: &str) -> Self {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        Self(
            argon2
                .hash_password(password.as_bytes(), &salt)
                .unwrap()
                .to_string(),
        )
    }

    pub fn password_valid(&self, password: &str) -> bool {
        let argon2 = Argon2::default();
        let parsed_hash = password_hash::PasswordHash::new(&self.0).unwrap();
        argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }
}

impl Display for PasswordHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for UserName {
    fn from(source: &str) -> Self {
        Self(source.trim().to_string())
    }
}

impl FromStr for Email {
    type Err = InvalidEmail;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let valid = s.contains('@') && !s.contains(|x: char| x.is_whitespace());
        if valid {
            Ok(Email(s.to_string()))
        } else {
            Err(InvalidEmail)
        }
    }
}
