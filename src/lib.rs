//! # Paddle API Client
//!
//! This is a Rust client for the Paddle API, which allows you to interact with Paddle's services.

use enums::TaxCategory;
use reqwest::{header::CONTENT_TYPE, IntoUrl, Method, Url};
use serde::{de::DeserializeOwned, Serialize};

pub mod entities;
pub mod enums;
pub mod error;
pub mod ids;

pub mod products;

pub mod response;
use response::{ErrorResponse, Response, SuccessResponse};

use error::{Error, PaddleError};

type Result<T> = std::result::Result<SuccessResponse<T>, Error>;

/// Paddle API client
///
/// This struct is used to create a new Paddle client instance.
#[derive(Clone, Debug)]
pub struct Paddle {
    base_url: Url,
    api_key: String,
}

impl Paddle {
    pub const PRODUCTION: &'static str = "https://api.paddle.com";
    pub const SANDBOX: &'static str = "https://sandbox-api.paddle.com";

    /// Creates a new Paddle client instance.
    ///
    /// Example:
    /// ```
    /// use paddle::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::PRODUCTION).unwrap();
    /// ```
    pub fn new(
        api_key: impl Into<String>,
        base_url: impl IntoUrl,
    ) -> std::result::Result<Self, Error> {
        Ok(Self {
            base_url: base_url.into_url()?,
            api_key: api_key.into(),
        })
    }

    /// Returns a request builder for fetching products. Use the after method to page through results.
    ///
    /// By default, Paddle returns products that are active. Use the status method to return products that are archived.
    /// Use the include method to include related price entities in the response.
    ///
    /// # Example:
    /// ```
    /// use paddle::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::PRODUCTION).unwrap();
    /// let products = client.products_list().send().await.unwrap();
    /// ```
    pub fn products_list(&self) -> products::ProductsList {
        products::ProductsList::new(self)
    }

    /// Returns a request builder for creating a new product.
    ///
    /// This method requires a name and a tax category.
    ///
    /// # Example:
    /// ```
    /// use paddle::Paddle;
    /// use paddle::enums::TaxCategory;
    /// let client = Paddle::new("your_api_key", Paddle::PRODUCTION).unwrap();
    /// let product = client.products_create("My Product", TaxCategory::Digital).send().await.unwrap();
    /// ```
    pub fn products_create(
        &self,
        name: impl Into<String>,
        tax_category: TaxCategory,
    ) -> products::ProductsCreate {
        products::ProductsCreate::new(self, name, tax_category)
    }

    async fn send<T: DeserializeOwned>(
        &self,
        req: impl Serialize,
        method: Method,
        path: &str,
    ) -> Result<T> {
        let url = self.base_url.join(path)?;
        let client = reqwest::Client::new();

        let mut builder = client
            .request(method.clone(), url)
            .bearer_auth(self.api_key.clone())
            .header(CONTENT_TYPE, "application/json; charset=utf-8");

        builder = match method {
            reqwest::Method::GET => builder.query(&req),
            reqwest::Method::POST => builder.json(&req),
            reqwest::Method::PUT => builder.json(&req),
            _ => builder,
        };

        let res: Response<_> = builder.send().await?.json().await?;

        match res {
            Response::Success(success) => Ok(success),
            Response::Error(error) => Err(Error::Paddle(error)),
        }
    }
}

fn comma_separated<S, T>(
    values: &Option<Vec<T>>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
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
