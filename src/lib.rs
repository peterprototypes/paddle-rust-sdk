use reqwest::{header::CONTENT_TYPE, IntoUrl, Url};
use serde::{Deserialize, Serialize};

pub mod entities;
pub mod enums;
pub mod error;
pub mod ids;

pub mod products;

use error::{Error, PaddleError};

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub request_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Response<T> {
    Success(SuccessResponse<T>),
    Error(ErrorResponse),
}

#[derive(Debug, Deserialize)]
pub struct SuccessResponse<T> {
    pub data: T,
    pub meta: Meta,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: PaddleError,
    pub meta: Meta,
}

// pub struct PaddleResponse<T> {
//     data: T,
//     meta: Meta,
// }

pub trait PaddleRequest: Serialize {
    type Response: serde::de::DeserializeOwned;

    fn path(&self) -> String;
    fn method(&self) -> reqwest::Method;
}

pub struct Paddle {
    base_url: Url,
    api_key: String,
}

impl Paddle {
    pub const PRODUCTION: &'static str = "https://api.paddle.com";
    pub const SANDBOX: &'static str = "https://sandbox-api.paddle.com";

    pub fn new(api_key: String, base_url: impl IntoUrl) -> Result<Self, Error> {
        Ok(Self {
            base_url: base_url.into_url()?,
            api_key,
        })
    }

    pub async fn send<T: PaddleRequest>(
        &self,
        req: T,
    ) -> Result<SuccessResponse<T::Response>, Error> {
        let url = self.base_url.join(&req.path())?;
        let client = reqwest::Client::new();

        let method = req.method();

        let mut builder = client
            .request(req.method(), url)
            .bearer_auth(self.api_key.clone())
            .header(CONTENT_TYPE, "application/json; charset=utf-8");

        builder = match method {
            reqwest::Method::GET => builder.query(&req),
            reqwest::Method::POST => builder.json(&req),
            reqwest::Method::PUT => builder.json(&req),
            _ => builder,
        };

        // let res = builder.send().await?.text().await?;
        // dbg!(res);
        // todo!()

        let res: Response<_> = builder.send().await?.json().await?;

        match res {
            Response::Success(success) => Ok(success),
            Response::Error(error) => Err(Error::Paddle(error)),
        }
    }
}

fn comma_separated<S, T>(values: &Option<Vec<T>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    T: AsRef<str>,
{
    match values {
        Some(values) => {
            let values = values
                .iter()
                .map(|v| v.as_ref())
                .collect::<Vec<_>>()
                .join(",");

            serializer.serialize_str(&values)
        }
        None => serializer.serialize_none(),
    }
}
