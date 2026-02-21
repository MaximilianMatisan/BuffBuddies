use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    ///Subject of the JWT: In our case username
    sub: String,
    ///Unix timestamp for an expiration date
    exp: usize,
    //Issued at
    //iat: usize,
}

pub fn create_jwt(username: String) -> String {
    let expiration_time = Utc::now().timestamp() + 1800;

    let claims = Claims {
        sub: username,
        exp: expiration_time as usize,
    };

    let encoded = encode::<Claims>(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("123".as_ref()), //TODO HIDE
    );
    encoded.unwrap()
}

pub fn decode_jwt(token: &str) -> Option<String> {
    //println!("Received {} token from client!", token);

    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = false; // Could be future enhancement

    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret("123".as_ref()), //TODO HIDE
        &validation,
    )
    .ok()?;
    //println!("The token data is valid if this is printed {:#?}", token_data);
    Some(token_data.claims.sub)
}
