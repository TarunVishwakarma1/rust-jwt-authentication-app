use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct User {
    pub email: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Claims {
    pub email: String,
    pub exp: i64,
}

pub fn get_jwt(user: User) -> Result<String, String>{
    let token = encode(
        &Header::default(),
        &Claims {
          email: user.email,
          exp: (Utc::now() + Duration::minutes(1)).timestamp(),
        },
        &EncodingKey::from_secret("mykey".as_bytes()),
      )
      .map_err(|e| e.to_string());
    
      return token;
}