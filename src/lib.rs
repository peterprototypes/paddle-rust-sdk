//! # Paddle API Client
//!
//! This is a Rust client for the Paddle API, which allows you to interact with Paddle's services.

use reqwest::{header::CONTENT_TYPE, IntoUrl, Method, Url};
use serde::{de::DeserializeOwned, Serialize};

pub mod entities;
pub mod enums;
pub mod error;
pub mod ids;

pub mod discounts;
pub mod prices;
pub mod products;

pub mod response;

use enums::{CurrencyCode, DiscountType, TaxCategory};
use ids::{DiscountID, PriceID, ProductID};

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
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// ```
    #[allow(clippy::result_large_err)]
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
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let products = client.products_list().send().await.unwrap();
    /// ```
    pub fn products_list(&self) -> products::ProductsList {
        products::ProductsList::new(self)
    }

    /// Returns a request builder for creating a new product.
    ///
    /// # Example:
    /// ```
    /// use paddle::Paddle;
    /// use paddle::enums::TaxCategory;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let product = client.products_create("My Product", TaxCategory::Standard).send().await.unwrap();
    /// ```
    pub fn product_create(
        &self,
        name: impl Into<String>,
        tax_category: TaxCategory,
    ) -> products::ProductCreate {
        products::ProductCreate::new(self, name, tax_category)
    }

    /// Returns a request builder for fetching a specific product.
    ///
    /// # Example:
    /// ```
    /// use paddle::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let product = client.product_get("pro_01jqx9rd...").send().await.unwrap();
    /// ```
    pub fn product_get(&self, product_id: impl Into<ProductID>) -> products::ProductGet {
        products::ProductGet::new(self, product_id)
    }

    /// Returns a request builder for updating a specific product.
    ///
    /// # Example:
    /// ```
    /// use paddle::Paddle;
    /// use paddle::enums::TaxCategory;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let product = client.product_update("pro_01jqx9rd...").name("My New Name").send().await.unwrap();
    /// ```
    pub fn product_update(&self, product_id: impl Into<ProductID>) -> products::ProductUpdate {
        products::ProductUpdate::new(self, product_id)
    }

    /// Returns a request builder listing prices
    ///
    /// # Example:
    /// ```
    /// use paddle::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let prices = client.prices_list().send().await.unwrap();
    /// ```
    pub fn prices_list(&self) -> prices::PricesList {
        prices::PricesList::new(self)
    }

    /// Returns a request builder for creating a new price.
    ///
    /// * `product_id` - Paddle ID for the product that this price is for.
    /// * `description` - Internal description for this price, not shown to customers. Typically notes for your team.
    /// * `amount` - Amount of the price in the smallest unit of the currency (e.g. 1000 cents for 10 USD).
    /// * `currency` - Currency code for the price. Use the [CurrencyCode] enum to specify the currency.
    ///
    /// # Example:
    /// ```
    /// use paddle::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let price = client.price_create("pro_01jqx9rd...", "Low price", 19.99, CurrencyCode::USD).send().await.unwrap();
    /// ```
    pub fn price_create(
        &self,
        product_id: impl Into<ProductID>,
        description: impl Into<String>,
        amount: u64,
        currency: CurrencyCode,
    ) -> prices::PricesCreate {
        prices::PricesCreate::new(self, product_id, description, amount, currency)
    }

    /// Returns a request builder for fetching a specific price by id.
    ///
    /// # Example:
    /// ```
    /// use paddle::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let price = client.price_get("price_01jqx9rd...").send().await.unwrap();
    /// ```
    pub fn price_get(&self, price_id: impl Into<PriceID>) -> prices::PriceGet {
        prices::PriceGet::new(self, price_id)
    }

    /// Returns a request builder for updating a specific price.
    ///
    /// # Example:
    /// ```
    /// use paddle::Paddle;
    /// use paddle::enums::TaxCategory;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let price = client.price_update("pri_01jqxv...").name("Updated Name").send().await.unwrap();
    /// ```
    pub fn price_update(&self, price_id: impl Into<PriceID>) -> prices::PriceUpdate {
        prices::PriceUpdate::new(self, price_id)
    }

    /// Returns a request builder for fetching discounts.
    ///
    /// # Example:
    /// ```
    /// use paddle::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let discounts = client.discounts_list().send().await.unwrap();
    /// ```
    pub fn discounts_list(&self) -> discounts::DiscountsList {
        discounts::DiscountsList::new(self)
    }

    /// Returns a request builder for creating discounts.
    ///
    /// # Example:
    /// ```
    /// use paddle::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let discount = client.discount_create("15", "Winter Holidays", DiscountType::Percentage).send().await.unwrap();
    /// ```
    pub fn discount_create(
        &self,
        amount: impl Into<String>,
        description: impl Into<String>,
        discount_type: DiscountType,
    ) -> discounts::DiscountCreate {
        discounts::DiscountCreate::new(self, amount, description, discount_type)
    }

    /// Returns a request builder for fetching a specific discount by id.
    ///
    /// # Example:
    /// ```
    /// use paddle::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let discount = client.discount_get("dsc_01jqzpbmnq...").send().await.unwrap();
    /// ```
    pub fn discount_get(&self, discount_id: impl Into<DiscountID>) -> discounts::DiscountGet {
        discounts::DiscountGet::new(self, discount_id)
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
            reqwest::Method::POST | reqwest::Method::PUT | reqwest::Method::PATCH => {
                builder.json(&req)
            }
            _ => builder,
        };

        // let text = builder.send().await?.text().await?;
        // println!("{}", text);
        // todo!();

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

// fn comma_separated_enum<S, T>(
//     values: &Vec<T>,
//     serializer: S,
// ) -> std::result::Result<S::Ok, S::Error>
// where
//     S: serde::Serializer,
//     T: Serialize,
// {
//     let mut serialized = vec![];

//     for val in values {
//         let serialized_value = serde_json::to_string(val).map_err(serde::ser::Error::custom)?;
//         serialized.push(serialized_value);
//     }

//     serializer.serialize_str(serialized.join(",").as_str())
// }
