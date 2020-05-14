use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

#[derive(serde::Serialize)]
pub struct Token {
    token: String,
}
#[derive(serde::Serialize)]
pub struct Jwt<S = &'static str> {
    /// Issuer (journali.nl)
    iss: S,

    /// expiration time
    exp: DateTime<Utc>,

    /// subject
    sub: Uuid,
}

impl Jwt {
    pub fn new(iss: &'static str, duration: Duration, sub: Uuid) -> Self {
        let now = Utc::now();
        let exp = now + duration;

        Self { iss, exp, sub }
    }

    pub fn tokenize(self) -> Token {
        use jsonwebtoken::{encode, EncodingKey, Header};

        let secret = std::env::var("SECRET").expect("SECRET");
        let token = encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(&secret.as_bytes()),
        )
        .unwrap();

        Token { token }
    }
}
