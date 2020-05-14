use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::errors::Error;
use uuid::Uuid;

#[derive(serde::Serialize)]
pub struct Token {
    token: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Jwt {
    /// Issuer (journali.nl)
    iss: String,

    /// expiration time
    exp: DateTime<Utc>,

    /// subject
    pub sub: Uuid,
}

fn get_secret() -> String {
    std::env::var("SECRET").expect("SECRET")
}

impl Jwt {
    pub fn new(iss: String, duration: Duration, sub: Uuid) -> Self {
        let now = Utc::now();
        let exp = now + duration;

        Self { iss, exp, sub }
    }

    pub fn tokenize(self) -> Token {
        use jsonwebtoken::{encode, EncodingKey, Header};

        let secret = get_secret();
        let token = encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(&secret.as_bytes()),
        )
        .unwrap();

        Token { token }
    }

    pub fn decrypt(jwt: &str) -> Result<Jwt, Error> {
        use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

        let secret = get_secret();

        decode::<Jwt>(
            jwt,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map(|token| token.claims)
    }
}
