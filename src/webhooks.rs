use std::num::ParseIntError;
use std::str::FromStr;

use chrono::{prelude::*, Duration};
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::error::{Error, SignatureError};

type HmacSha256 = Hmac<Sha256>;

pub struct MaximumVariance(pub Option<Duration>);

impl MaximumVariance {
    pub fn seconds(seconds: u64) -> Self {
        Self(Some(Duration::seconds(seconds as i64)))
    }
}

impl Default for MaximumVariance {
    fn default() -> Self {
        Self(Some(Duration::seconds(5)))
    }
}

pub struct Signature {
    timestamp: DateTime<Utc>,
    signature: Vec<u8>,
}

impl Signature {
    pub fn verify(
        &self,
        request_body: impl AsRef<str>,
        key: impl AsRef<str>,
        maximum_variance: MaximumVariance,
    ) -> Result<(), Error> {
        if let Some(maximum_variance) = maximum_variance.0 {
            if Utc::now() > self.timestamp + maximum_variance {
                return Err(Error::PaddleSignature(SignatureError::MaxVarianceExceeded(
                    maximum_variance,
                )));
            }
        }

        let signed_payload = format!("{}:{}", self.timestamp.format("%s"), request_body.as_ref());

        let mut mac = HmacSha256::new_from_slice(key.as_ref().as_bytes())
            .expect("HMAC can take key of any size");

        mac.update(signed_payload.as_bytes());
        mac.verify_slice(&self.signature)?;

        Ok(())
    }
}

impl FromStr for Signature {
    type Err = crate::Error;

    fn from_str(signature: &str) -> Result<Self, Self::Err> {
        // ts=1671552777;h1=eb4d0dc8853be92b7f063b9f3ba5233eb920a09459b6e6b2c26705b4364db151

        if signature.is_empty() {
            return Err(Error::PaddleSignature(SignatureError::Empty));
        }

        let signature_parts = signature.split(';').collect::<Vec<_>>();

        if signature_parts.len() != 2 {
            return Err(Error::PaddleSignature(SignatureError::InvalidFormat));
        }

        let mut timestamp = None;
        let mut signature = None;

        for part in signature_parts {
            let key_value = part.split('=').collect::<Vec<_>>();

            if key_value.len() != 2 {
                return Err(Error::PaddleSignature(SignatureError::InvalidPartFormat));
            }

            if key_value[0] == "ts" {
                timestamp = DateTime::from_timestamp(key_value[1].parse()?, 0);
            }

            if key_value[0] == "h1" {
                signature = Some(key_value[1].to_string());
            }
        }

        let Some((timestamp, signature)) = timestamp.zip(signature) else {
            return Err(Error::PaddleSignature(SignatureError::InvalidFormat));
        };

        Ok(Self {
            timestamp,
            signature: decode_hex(&signature)?,
        })
    }
}

fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_correctly() {
        let signature_str =
            "ts=1671552777;h1=eb4d0dc8853be92b7f063b9f3ba5233eb920a09459b6e6b2c26705b4364db151";

        let sig: Signature = signature_str.parse().expect("To parse correctly");
    }

    #[test]
    fn malformed_parameters() {
        let signature_str =
            "ts=1671552777h1=eb4d0dc8853be92b7f063b9f3ba5233eb920a09459b6e6b2c26705b4364db151";
        assert!(signature_str.parse::<Signature>().is_err());

        let signature_str =
            "ts=1671552a777;h1=eb4d0dc8853be92b7f063b9f3ba5233eb920a09459b6e6b2c26705b4364db151";
        assert!(signature_str.parse::<Signature>().is_err());
    }
}
