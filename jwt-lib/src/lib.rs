use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Header, Encoding};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct User {
    pub email: String,
}

struct Claims {
    pub email: String,
    pub exp: i64,
}

pub fn get_jwt(user: User){
    let token = encode(&Header::default(), &Claims{
        email: user.email,
        exp: (Utc::now() + Duration::minutes(1)).timestamp(),
    },&Encoding::from_secret("mykey".as_bytes()));
}