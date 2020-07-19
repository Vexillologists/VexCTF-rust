use serde::{Deserialize, Serialize};
use std::env;

use r2d2_redis::redis;
use redis::{Commands, Connection, RedisResult};

use frank_jwt::{
    decode as decode_jwt, encode as encode_jwt, error::Error as JWTError, Algorithm,
    ValidationOptions,
};

use serde_json::Error as JSONError;

use chrono::prelude::*;
use chrono::Duration;

use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub exp: i64,
    pub uid: i32,
    pub jti: u64,
}

impl Token {
    pub fn verify(&self, conn: &mut Connection) -> RedisResult<bool> {
        conn.exists(format!("validSessions:{}", self.jti))
    }

    pub fn revoke(&self, conn: &mut Connection) -> RedisResult<()> {
        conn.del(format!("validSessions:{}", self.jti))?;
        Ok(())
    }

    pub fn encode(&self) -> Result<String, JWTError> {
        let header = json!({});
        let secret = cookie_secret();
        let payload = serde_json::to_value(&self).unwrap();
        encode_jwt(header, &secret, &payload, Algorithm::HS256)
    }

    pub fn generate(uid: i32, conn: &mut Connection) -> RedisResult<Self> {
        let exp = (Utc::now() + Duration::hours(36)).timestamp();
        let jti = super::crypto::secure_random64();

        conn.set(format!("validSessions:{}", jti), "1")?;
        conn.expire(format!("validSessions:{}", jti), 36 * 60 * 60)?;

        Ok(Self { jti, exp, uid })
    }

    pub fn decode(jwt: &str) -> Result<Self, JWTErrorKind> {
        let secret = cookie_secret();
        let (_, payload) = match decode_jwt(
            &jwt,
            &secret,
            Algorithm::HS256,
            &ValidationOptions::default(),
        ) {
            Ok(v) => v,
            Err(e) => {
                return Err(JWTErrorKind::JWTError(e));
            }
        };
        match serde_json::from_value(payload) {
            Ok(v) => Ok(v),
            Err(e) => Err(JWTErrorKind::JSONError(e)),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let jwt = self.encode();
        if jwt.is_err() {
            return Err(std::fmt::Error);
        }
        let jwt = jwt.unwrap();
        write!(f, "{}", jwt)
    }
}

#[derive(Debug)]
pub enum JWTErrorKind {
    JWTError(JWTError),
    JSONError(JSONError),
}

fn cookie_secret() -> String {
    env::var("COOKIE_SECRET").expect("COOKIE_SECRET must be set")
}
