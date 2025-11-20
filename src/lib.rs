#![allow(clippy::result_large_err)]

//! # Paddle API Client
//!
//! An async client library for interaction with Paddle API, An async and ergonomic wrapper around Paddle's REST HTTP API.
//!
//! Every interaction is done via the [Paddle] client type.
//!
//! ## Init and Usage Example
//!
//! ```rust,no_run
//! use paddle_rust_sdk::Paddle;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Paddle::new(std::env::var("PADDLE_API_KEY")?, Paddle::SANDBOX)?;
//!
//!     let mut list = client.customers_list();
//!     let mut paginated = list.per_page(2).send();
//!     let customers = paginated.all().await?;
//!
//!     dbg!(customers);
//!
//!     Ok(())
//! }
//! ```
//!
//! The `examples/` dir has up to date working example code.
//!
//! ## Webhook signature verification
//!
//! Use the [Paddle::unmarshal] method to verify that received events are genuinely sent from Paddle. Additionally, this method returns the deserialized event struct.
//!

use paddle_rust_sdk_types::reports::ReportType;
pub use paddle_rust_sdk_types::{entities, enums, ids};
use reqwest::{header::CONTENT_TYPE, IntoUrl, Method, StatusCode, Url};
use serde::{de::DeserializeOwned, Serialize};

pub mod error;
pub mod webhooks;

pub mod addresses;
pub mod adjustments;
pub mod businesses;
pub mod customers;
pub mod discounts;
pub mod events;
pub mod paginated;
pub mod payment_methods;
pub mod prices;
pub mod pricing_preview;
pub mod products;
pub mod reports;
pub mod subscriptions;
pub mod transactions;

pub mod response;

use paddle_rust_sdk_types::entities::{
    CustomerAuthenticationToken, Event, EventType, PricePreviewItem, ReportBase, Subscription,
    Transaction, TransactionInvoice,
};
use paddle_rust_sdk_types::enums::{
    AdjustmentAction, CountryCodeSupported, CurrencyCode, DiscountType, Disposition, TaxCategory,
};
use paddle_rust_sdk_types::ids::{
    AddressID, AdjustmentID, BusinessID, CustomerID, DiscountID, PaddleID, PaymentMethodID,
    PriceID, ProductID, SubscriptionID, TransactionID,
};
use webhooks::{MaximumVariance, Signature};

use error::PaddleApiError;
use response::{ErrorResponse, Response, SuccessResponse};

pub use error::Error;

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

    /// List of IP addresses Paddle uses to call webhook endpoints from the Live environment
    pub const ALLOWED_WEBHOOK_IPS_PRODUCTION: [&str; 6] = [
        "34.232.58.13",
        "34.195.105.136",
        "34.237.3.244",
        "35.155.119.135",
        "52.11.166.252",
        "34.212.5.7",
    ];

    /// List of IP addresses Paddle uses to call webhook endpoints from the Sandbox environment
    pub const ALLOWED_WEBHOOK_IPS_SANDBOX: [&str; 6] = [
        "34.194.127.46",
        "54.234.237.108",
        "3.208.120.145",
        "44.226.236.210",
        "44.241.183.62",
        "100.20.172.113",
    ];

    /// Creates a new Paddle client instance.
    ///
    /// Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
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

    /// Validate the integrity of a Paddle webhook request.
    ///
    /// - **request_body** - The raw body of the request. Don't transform or process the raw body of the request, including adding whitespace or applying other formatting. This results in a different signed payload, meaning signatures won't match when you compare.
    /// - **secret_key** - Secret key created in Paddle dashboard. Each notification destination has it's own secret key.
    /// - **signature** - "Paddle-Signature" HTTP request header from an incoming webhook sent by Paddle.
    /// - **maximum_variance** - Maximum allowed age for a generated signature. [MaximumVariance::default] is 5 seconds. Pass `MaximumVariance(None)` to disable timestamp checking.
    ///
    /// **Return** - the deserialized [Event] struct.
    ///
    /// The `examples/` directory contains a demo webhook handler for Actix web.
    pub fn unmarshal(
        request_body: impl AsRef<str>,
        secret_key: impl AsRef<str>,
        signature: impl AsRef<str>,
        maximum_variance: MaximumVariance,
    ) -> std::result::Result<Event, Error> {
        let signature: Signature = signature.as_ref().parse()?;
        signature.verify(request_body.as_ref(), secret_key, maximum_variance)?;

        let event = serde_json::from_str(request_body.as_ref())?;

        Ok(event)
    }

    /// Get a request builder for fetching products. Use the after method to page through results.
    ///
    /// By default, Paddle returns products that are active. Use the status method to return products that are archived.
    /// Use the include method to include related price entities in the response.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    ///
    /// let mut products_list = client.products_list();
    /// let mut products = products_list.order_by_asc("id").per_page(20).send();
    ///
    /// while let Some(res) = products.next().await.unwrap() {
    ///     dbg!(res.data);
    /// }
    /// ```
    pub fn products_list(&self) -> products::ProductsList<'_> {
        products::ProductsList::new(self)
    }

    /// Get a request builder for creating a new product.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// use paddle_rust_sdk::enums::TaxCategory;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let product = client.products_create("My Product", TaxCategory::Standard).send().await.unwrap();
    /// ```
    pub fn product_create(
        &self,
        name: impl Into<String>,
        tax_category: TaxCategory,
    ) -> products::ProductCreate<'_> {
        products::ProductCreate::new(self, name, tax_category)
    }

    /// Get a request builder for fetching a specific product.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let product = client.product_get("pro_01jqx9rd...").send().await.unwrap();
    /// ```
    pub fn product_get(&self, product_id: impl Into<ProductID>) -> products::ProductGet<'_> {
        products::ProductGet::new(self, product_id)
    }

    /// Get a request builder for updating a specific product.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// use paddle_rust_sdk::enums::TaxCategory;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let product = client.product_update("pro_01jqx9rd...").name("My New Name").send().await.unwrap();
    /// ```
    pub fn product_update(&self, product_id: impl Into<ProductID>) -> products::ProductUpdate<'_> {
        products::ProductUpdate::new(self, product_id)
    }

    /// Get a request builder listing prices
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    ///
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    ///
    /// let mut prices_list = client.prices_list();
    /// let mut prices = prices_list.order_by_asc("id").per_page(20).send();
    ///
    /// while let Some(res) = prices.next().await.unwrap() {
    ///     dbg!(res.data);
    /// }
    /// ```
    pub fn prices_list(&self) -> prices::PricesList<'_> {
        prices::PricesList::new(self)
    }

    /// Get a request builder for creating a new price.
    ///
    /// * `product_id` - Paddle ID for the product that this price is for.
    /// * `description` - Internal description for this price, not shown to customers. Typically notes for your team.
    /// * `amount` - Amount of the price in the smallest unit of the currency (e.g. 1000 cents for 10 USD).
    /// * `currency` - Currency code for the price. Use the [CurrencyCode] enum to specify the currency.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let price = client.price_create("pro_01jqx9rd...", "Low price", 19.99, CurrencyCode::USD).send().await.unwrap();
    /// ```
    pub fn price_create(
        &self,
        product_id: impl Into<ProductID>,
        description: impl Into<String>,
        amount: u64,
        currency: CurrencyCode,
    ) -> prices::PricesCreate<'_> {
        prices::PricesCreate::new(self, product_id, description, amount, currency)
    }

    /// Get a request builder for fetching a specific price by id.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let price = client.price_get("price_01jqx9rd...").send().await.unwrap();
    /// ```
    pub fn price_get(&self, price_id: impl Into<PriceID>) -> prices::PriceGet<'_> {
        prices::PriceGet::new(self, price_id)
    }

    /// Get a request builder for updating a specific price.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// use paddle_rust_sdk::enums::TaxCategory;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let price = client.price_update("pri_01jqxv...").name("Updated Name").send().await.unwrap();
    /// ```
    pub fn price_update(&self, price_id: impl Into<PriceID>) -> prices::PriceUpdate<'_> {
        prices::PriceUpdate::new(self, price_id)
    }

    /// Get a request builder for fetching discounts.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    ///
    /// let list = client.discounts_list();
    /// let mut discounts = list.send();
    ///
    /// while let Some(res) = discounts.next().await.unwrap() {
    ///     dbg!(res.data);
    /// }
    /// ```
    pub fn discounts_list(&self) -> discounts::DiscountsList<'_> {
        discounts::DiscountsList::new(self)
    }

    /// Get a request builder for creating discounts.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let discount = client.discount_create("15", "Winter Holidays", DiscountType::Percentage).send().await.unwrap();
    /// ```
    pub fn discount_create(
        &self,
        amount: impl Into<String>,
        description: impl Into<String>,
        discount_type: DiscountType,
    ) -> discounts::DiscountCreate<'_> {
        discounts::DiscountCreate::new(self, amount, description, discount_type)
    }

    /// Get a request builder for fetching a specific discount by id.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let discount = client.discount_get("dsc_01jqzpbmnq...").send().await.unwrap();
    /// ```
    pub fn discount_get(&self, discount_id: impl Into<DiscountID>) -> discounts::DiscountGet<'_> {
        discounts::DiscountGet::new(self, discount_id)
    }

    /// Get a request builder for creating discounts.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let discount = client.discount_update("dsc_01jqzpbmnq...").amount("18").send().await.unwrap();
    /// ```
    pub fn discount_update(
        &self,
        discount_id: impl Into<DiscountID>,
    ) -> discounts::DiscountUpdate<'_> {
        discounts::DiscountUpdate::new(self, discount_id)
    }

    /// Get a request builder for fetching customers. Use the after method to page through results.
    ///
    /// By default, Paddle returns customers that are `active`. Use the status query parameter to return customers that are archived.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let customers = client.customers_list().send().await.unwrap();
    /// ```
    pub fn customers_list(&self) -> customers::CustomersList<'_> {
        customers::CustomersList::new(self)
    }

    /// Get a request builder for creating a new customer.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let customers = client.customer_create("test@example.com").send().await.unwrap();
    /// ```
    pub fn customer_create(&self, email: impl Into<String>) -> customers::CustomerCreate<'_> {
        customers::CustomerCreate::new(self, email.into())
    }

    /// Get a request builder for fetching a specific customer by id.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let discount = client.customer_get("ctm_01jqztc78e1xfdgwhcgjzdrvgd").send().await.unwrap();
    /// ```
    pub fn customer_get(&self, customer_id: impl Into<CustomerID>) -> customers::CustomerGet<'_> {
        customers::CustomerGet::new(self, customer_id)
    }

    /// Get a request builder for updating customer data.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let discount = client.customer_update("ctm_01jqztc78e1xfdgwhcgjzdrvgd").email("new_email@example.com").send().await.unwrap();
    /// ```
    pub fn customer_update(
        &self,
        customer_id: impl Into<CustomerID>,
    ) -> customers::CustomerUpdate<'_> {
        customers::CustomerUpdate::new(self, customer_id)
    }

    /// Get a request builder for fetching a list of credit balances for each currency for a customer.
    ///
    /// Each balance has three totals:
    ///
    /// * `available` - total available to use.
    /// * `reserved` - total temporarily reserved for billed transactions.
    /// * `used` - total amount of credit used.
    ///
    /// Credit is added to the available total initially. When used, it moves to the used total.
    ///
    /// The reserved total is used when a credit balance is applied to a transaction that's marked as billed, like when working with an issued invoice. It's not available for other transactions at this point, but isn't considered used until the transaction is completed. If a billed transaction is canceled, any reserved credit moves back to available.
    ///
    /// Credit balances are created automatically by Paddle when you take an action that results in Paddle creating a credit for a customer, like making prorated changes to a subscription. An empty data array is returned where a customer has no credit balances.
    ///
    /// The response is not paginated.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let discount = client.customer_credit_balances("ctm_01jqztc78e1xfdgwhcgjzdrvgd").send().await.unwrap();
    /// ```
    pub fn customer_credit_balances(
        &self,
        customer_id: impl Into<CustomerID>,
    ) -> customers::CustomerCreditBalances<'_> {
        customers::CustomerCreditBalances::new(self, customer_id)
    }

    /// Generates an authentication token for a customer.
    ///
    /// You can pass a generated authentication token to Paddle.js when opening a checkout to let customers work with saved payment methods.
    ///
    /// Authentication tokens are temporary and shouldn't be cached. They're valid until the expires_at date returned in the response.
    pub async fn generate_auth_token(
        &self,
        customer_id: impl Into<CustomerID>,
    ) -> Result<CustomerAuthenticationToken> {
        let client = reqwest::Client::new();

        let customer_id = customer_id.into();

        let url = format!(
            "{}customers/{}/auth-token",
            self.base_url,
            customer_id.as_ref()
        );

        let res: Response<_> = client
            .post(url)
            .bearer_auth(self.api_key.clone())
            .send()
            .await?
            .json()
            .await?;

        match res {
            Response::Success(success) => Ok(success),
            Response::Error(error) => Err(Error::PaddleApi(error)),
        }
    }

    /// Get a request builder for fetching customers addresses.
    ///
    /// By default, Paddle returns addresses that are `active`. Use the status query parameter to return addresses that are archived.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let customers = client.addresses_list("ctm_01jqztc78e1xfdgwhcgjzdrvgd").send().await.unwrap();
    /// ```
    pub fn addresses_list(
        &self,
        customer_id: impl Into<CustomerID>,
    ) -> addresses::AddressesList<'_> {
        addresses::AddressesList::new(self, customer_id)
    }

    /// Get a request builder for creating a new customer address.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let customers = client.address_create("ctm_01jqztc78e1xfdgwhcgjzdrvgd", CountryCodeSupported::US).send().await.unwrap();
    /// ```
    pub fn address_create(
        &self,
        customer_id: impl Into<CustomerID>,
        country_code: CountryCodeSupported,
    ) -> addresses::AddressCreate<'_> {
        addresses::AddressCreate::new(self, customer_id, country_code)
    }

    /// Get a request builder for getting an address for a customer using its ID and related customer ID.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let customers = client.address_get("ctm_01jqztc78e1xfdgwhcgjzdrvgd", "add_01hv8gwdfkw5z6d1yy6pa3xyrz").send().await.unwrap();
    /// ```
    pub fn address_get(
        &self,
        customer_id: impl Into<CustomerID>,
        address_id: impl Into<AddressID>,
    ) -> addresses::AddressGet<'_> {
        addresses::AddressGet::new(self, customer_id, address_id)
    }

    /// Get a request builder for updating an address for a customer using its ID and related customer ID.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let customers = client.address_update("ctm_01jqztc78e1xfdgwhcgjzdrvgd", "add_01hv8gwdfkw5z6d1yy6pa3xyrz").first_line("Test").send().await.unwrap();
    /// ```
    pub fn address_update(
        &self,
        customer_id: impl Into<CustomerID>,
        address_id: impl Into<AddressID>,
    ) -> addresses::AddressUpdate<'_> {
        addresses::AddressUpdate::new(self, customer_id, address_id)
    }

    /// Get a request builder for fetching customers businesses.
    ///
    /// By default, Paddle returns addresses that are `active`. Use the status query parameter to return businesses that are archived.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let customers = client.businesses_list("ctm_01jqztc78e1xfdgwhcgjzdrvgd").send().await.unwrap();
    /// ```
    pub fn businesses_list(
        &self,
        customer_id: impl Into<CustomerID>,
    ) -> businesses::BusinessesList<'_> {
        businesses::BusinessesList::new(self, customer_id)
    }

    /// Get a request builder for creating a new customer business.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let customers = client.business_create("ctm_01jqztc78e1xfdgwhcgjzdrvgd", "Company Inc.").send().await.unwrap();
    /// ```
    pub fn business_create(
        &self,
        customer_id: impl Into<CustomerID>,
        name: impl Into<String>,
    ) -> businesses::BusinessCreate<'_> {
        businesses::BusinessCreate::new(self, customer_id, name)
    }

    /// Get a request builder for getting a business for a customer using its ID and related customer ID.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let customers = client.business_get("ctm_01jqztc78e1xfdgwhcgjzdrvgd", "biz_01jr85bypq4d3w139m53zw2559").send().await.unwrap();
    /// ```
    pub fn business_get(
        &self,
        customer_id: impl Into<CustomerID>,
        business_id: impl Into<BusinessID>,
    ) -> businesses::BusinessGet<'_> {
        businesses::BusinessGet::new(self, customer_id, business_id)
    }

    /// Get a request builder for updating a business for a customer using its ID and related customer ID.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let customers = client.business_update("ctm_01jqztc78e1xfdgwhcgjzdrvgd", "biz_01jr85bypq4d3w139m53zw2559").first_line("Test").send().await.unwrap();
    /// ```
    pub fn business_update(
        &self,
        customer_id: impl Into<CustomerID>,
        business_id: impl Into<BusinessID>,
    ) -> businesses::BusinessUpdate<'_> {
        businesses::BusinessUpdate::new(self, customer_id, business_id)
    }

    /// Get a request builder for querying customer saved payment methods.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let customers = client.payment_methods_list("ctm_01jqztc78e1xfdgwhcgjzdrvgd").send().await.unwrap();
    /// ```
    pub fn payment_methods_list(
        &self,
        customer_id: impl Into<CustomerID>,
    ) -> payment_methods::PaymentMethodsList<'_> {
        payment_methods::PaymentMethodsList::new(self, customer_id)
    }

    /// Get a request builder for getting a saved payment for a customer using its ID and related customer ID.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let customers = client.payment_method_get("ctm_01jqztc78e1xfdgwhcgjzdrvgd", "paymtd_01j2jff1m3es31sdkejpaym164").send().await.unwrap();
    /// ```
    pub fn payment_method_get(
        &self,
        customer_id: impl Into<CustomerID>,
        payment_method_id: impl Into<PaymentMethodID>,
    ) -> payment_methods::PaymentMethodGet<'_> {
        payment_methods::PaymentMethodGet::new(self, customer_id, payment_method_id)
    }

    /// Deletes a customer payment method using its ID.
    ///
    /// When you delete a customer payment method, it's permanently removed from that customer.
    ///
    /// There's no way to recover a deleted payment method.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// client.payment_method_delete("ctm_01jqztc78e1xfdgwhcgjzdrvgd", "paymtd_01j2jff1m3es31sdkejpaym164").await.unwrap();
    /// ```
    pub async fn payment_method_delete(
        &self,
        customer_id: impl Into<CustomerID>,
        payment_method_id: impl Into<PaymentMethodID>,
    ) -> std::result::Result<bool, Error> {
        let client = reqwest::Client::new();

        let url = format!(
            "{}customers/{}/payment-methods/{}",
            self.base_url,
            customer_id.into().as_ref(),
            payment_method_id.into().as_ref()
        );

        let response = client
            .delete(url)
            .bearer_auth(self.api_key.clone())
            .send()
            .await?;

        Ok(response.status() == StatusCode::NO_CONTENT)
    }

    /// Creates a customer portal session for a customer.
    ///
    /// You can use this to generate authenticated links for a customer so that they're automatically signed in to the portal. Typically used when linking to the customer portal from your app where customers are already authenticated.
    ///
    /// You can include an array of subscription_ids to generate authenticated portal links that let customers make changes to their subscriptions. You can use these links as part of subscription management workflows rather than building your own billing screens.
    ///
    /// Customer portal sessions are temporary and shouldn't be cached.
    ///
    /// The customer portal is fully hosted by Paddle. For security and the best customer experience, don't embed the customer portal in an iframe.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let session = client.create_portal_session("ctm_01jqztc78e1xfdgwhcgjzdrvgd").send().await.unwrap();
    /// dbg!(session.data.urls.general.overview);
    /// dbg!(session.data.urls.subscriptions);
    /// ```
    pub fn create_portal_session(
        &self,
        customer_id: impl Into<CustomerID>,
    ) -> customers::PortalSessionCreate<'_> {
        customers::PortalSessionCreate::new(self, customer_id)
    }

    /// Get a request builder for querying transactions.
    ///
    /// Use the include method on the builder to include related entities in the response.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let transactions = client.transactions_list().send().await.unwrap();
    /// ```
    pub fn transactions_list(&self) -> transactions::TransactionsList<'_> {
        transactions::TransactionsList::new(self)
    }

    /// Get a request builder for creating a transaction.
    ///
    /// See [Create Transaction](https://developer.paddle.com/api-reference/transactions/create-transaction) for more information.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    ///
    /// let res = client.transaction_create()
    ///     .append_catalog_item("pri_01jqxvdyjkp961jzv4me7ezg4d", 1)
    ///     .send()
    ///     .await
    ///     .unwrap();
    ///
    /// dbg!(res.data);
    /// ```
    pub fn transaction_create(&self) -> transactions::TransactionCreate<'_> {
        transactions::TransactionCreate::new(self)
    }

    /// Get a request builder for fetching a transaction using its ID.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let res = client.transaction_get("txn_01hv8wptq8987qeep44cyrewp9").send().await.unwrap();
    /// dbg!(res.data);
    /// ```
    pub fn transaction_get(
        &self,
        transaction_id: impl Into<TransactionID>,
    ) -> transactions::TransactionGet<'_> {
        transactions::TransactionGet::new(self, transaction_id)
    }

    /// Get a request builder for updating a transaction.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::{enums::TransactionStatus, Paddle};
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// client.transaction_update("txn_01hv8wptq8987qeep44cyrewp9").status(TransactionStatus::Billed).send().await.unwrap();
    /// ```
    pub fn transaction_update(
        &self,
        transaction_id: impl Into<TransactionID>,
    ) -> transactions::TransactionUpdate<'_> {
        transactions::TransactionUpdate::new(self, transaction_id)
    }

    /// Returns a link to an invoice PDF for a transaction.
    ///
    /// Invoice PDFs are available for both automatically and manually-collected transactions:
    ///   - The PDF for manually-collected transactions includes payment terms, purchase order number, and notes for your customer. It's a demand for payment from your customer. It's available for transactions that are `billed` or `completed`.
    ///   - The PDF for automatically-collected transactions lets your customer know that payment was taken successfully. Customers may require this for for tax-reporting purposes. It's available for transactions that are `completed`.
    ///
    /// Invoice PDFs aren't available for zero-value transactions.
    ///
    /// The link returned is not a permanent link. It expires after an hour.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::{enums::Disposition, Paddle};
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let res = client.transaction_invoice("txn_01hv8wptq8987qeep44cyrewp9", Disposition::Inline).await.unwrap();
    /// dbg!(res.data.url)
    /// ```
    pub async fn transaction_invoice(
        &self,
        transaction_id: impl Into<TransactionID>,
        disposition: Disposition,
    ) -> Result<TransactionInvoice> {
        let transaction_id = transaction_id.into();

        let url = format!("/transactions/{}/invoice", transaction_id.as_ref());
        let params = ("disposition", disposition);

        self.send(params, Method::GET, &url).await
    }

    /// Get a request builder for generating a transaction preview without creating a transaction entity. Typically used for creating more advanced, dynamic pricing pages where users can build their own plans.
    ///
    /// You can provide location information when previewing a transaction. You must provide this if you want Paddle to calculate tax or automatically localize prices. You can provide one of:
    ///   - `customer_ip_address`: Paddle fetches location using the IP address to calculate totals.
    ///   - `address`: Paddle uses the country and ZIP code (where supplied) to calculate totals.
    ///   - `customer_id`, `address_id`, `business_id`: Paddle uses existing customer data to calculate totals. Typically used for logged-in customers.
    ///
    /// When supplying items, you can exclude items from the total calculation using the `include_in_totals` boolean.
    ///
    /// By default, recurring items with trials are considered to have a zero charge when previewing. Set `ignore_trials` to true to ignore trial periods against prices for transaction preview calculations.
    ///
    /// If successful, your response includes the data you sent with a details object that includes totals for the supplied prices.
    ///
    /// Transaction previews don't create transactions, so no `id` is returned.
    pub fn transaction_preview(&self) -> transactions::TransactionPreview<'_> {
        transactions::TransactionPreview::new(self)
    }

    /// Get a request builder to revise customer information for a billed or completed transaction.
    ///
    /// Revise a transaction to rectify incorrect customer, address, or business information on invoice documents generated by Paddle.
    ///
    /// You can revise transaction details that don't impact the tax rates on a transaction. This includes:
    ///   - Customer name
    ///   - Business name and tax or VAT number (`tax_identifier`)
    ///   - Address details, apart from the country
    ///
    /// You can't remove a valid tax or VAT number, only replace it with another valid one. If a valid tax or VAT number is added, Paddle automatically creates an adjustment to refund any tax where applicable.
    ///
    /// Transactions can only be revised once.
    ///
    /// If successful, your response includes a copy of the transaction entity. Get a transaction using the `include` parameter with the `customer`, `address`, and `business` values to see the revised customer information.
    ///
    /// Only the customer information for this transaction is updated. The related customer, address, and business entities aren't updated.
    pub fn transaction_revise(
        &self,
        transaction_id: impl Into<TransactionID>,
    ) -> transactions::TransactionRevise<'_> {
        transactions::TransactionRevise::new(self, transaction_id)
    }

    /// Get a request builder for querying subscriptions.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let subscriptions = client.subscriptions_list().send().await.unwrap();
    /// ```
    pub fn subscriptions_list(&self) -> subscriptions::SubscriptionsList<'_> {
        subscriptions::SubscriptionsList::new(self)
    }

    /// Get a request builder for fetching a subscription using its ID.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let res = client.subscription_get("sub_01hv8y5ehszzq0yv20ttx3166y").send().await.unwrap();
    /// dbg!(res.data);
    /// ```
    pub fn subscription_get(
        &self,
        subscription_id: impl Into<SubscriptionID>,
    ) -> subscriptions::SubscriptionGet<'_> {
        subscriptions::SubscriptionGet::new(self, subscription_id)
    }

    /// Get a request builder for getting a preview of changes to a subscription without actually applying them.
    ///
    /// Typically used for previewing proration before making changes to a subscription.
    ///
    /// If successful, your response includes `immediate_transaction`, `next_transaction`, and `recurring_transaction_details` so you can see expected transactions for the changes.
    ///
    /// The `update_summary` object contains details of prorated credits and charges created, along with the overall result of the update.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    ///
    /// let res = client.subscription_preview_update("sub_01hv8y5ehszzq0yv20ttx3166y")
    ///     .next_billed_at(Utc::now() + Days::new(10))
    ///     .proration_billing_mode(ProrationBillingMode::ProratedImmediately)
    ///     .send()
    ///     .await
    ///     .unwrap();
    ///
    /// dbg!(res.data);
    /// ```
    pub fn subscription_preview_update(
        &self,
        subscription_id: impl Into<SubscriptionID>,
    ) -> subscriptions::SubscriptionPreviewUpdate<'_> {
        subscriptions::SubscriptionPreviewUpdate::new(self, subscription_id)
    }

    /// Get a request builder for updating a subscription using its ID.
    ///
    /// When making changes to items or the next billing date for a subscription, you must include the `proration_billing_mode` field to tell Paddle how to bill for those changes.
    ///
    /// Send the complete list of items that you'd like to be on a subscription — including existing items. If you omit items, they're removed from the subscription.
    ///
    /// For each item, send `price_id` and `quantity`. Paddle responds with the full price object for each price. If you're updating an existing item, you can omit the `quantity` if you don't want to update it.
    ///
    /// If successful, your response includes a copy of the updated subscription entity. When an update results in an immediate charge, responses may take longer than usual while a payment attempt is processed.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let res = client.subscription_get("sub_01hv8y5ehszzq0yv20ttx3166y").send().await.unwrap();
    /// dbg!(res.data);
    /// ```
    pub fn subscription_update(
        &self,
        subscription_id: impl Into<SubscriptionID>,
    ) -> subscriptions::SubscriptionUpdate<'_> {
        subscriptions::SubscriptionUpdate::new(self, subscription_id)
    }

    /// Returns a transaction that you can pass to a checkout to let customers update their payment details. Only for subscriptions where collection_mode is automatic.
    ///
    /// The transaction returned depends on the status of the related subscription:
    /// - Where a subscription is `past_due`, it returns the most recent `past_due` transaction.
    /// - Where a subscription is `active`, it creates a new zero amount transaction for the items on a subscription.
    ///
    /// You can use the returned `checkout.url`, or pass the returned transaction ID to Paddle.js to open a checkout to present customers with a way of updating their payment details.
    ///
    /// The `customer`, `address`, `business`, `discount`, `adjustments` and `adjustments_totals` properties are only returned in the response if the API key has read permissions for those related entities.
    pub async fn subscription_update_payment_method_transaction(
        &self,
        subscription_id: impl Into<SubscriptionID>,
    ) -> Result<Transaction> {
        let subscription_id = subscription_id.into();

        let url = format!(
            "/subscriptions/{}/update-payment-method-transaction",
            subscription_id.as_ref()
        );

        self.send((), Method::GET, &url).await
    }

    /// Get a request builder for previewing creating a one-time charge for a subscription without billing that charge. Typically used for previewing calculations before making changes to a subscription.
    ///
    /// One-time charges are non-recurring items. These are price entities where the `billing_cycle` is `null`.
    ///
    /// If successful, your response includes `immediate_transaction`, `next_transaction`, and `recurring_transaction_details` so you can see expected transactions for the changes.
    pub fn subscription_preview_one_time_charge(
        &self,
        subscription_id: impl Into<SubscriptionID>,
    ) -> subscriptions::SubscriptionOneTimeChargePreview<'_> {
        subscriptions::SubscriptionOneTimeChargePreview::new(self, subscription_id)
    }

    /// Get a request builder for creating a new one-time charge for a subscription. Use to bill non-recurring items to a subscription. Non-recurring items are price entities where the `billing_cycle` is `null`.
    ///
    /// If successful, Paddle responds with the updated subscription entity. However, one-time charges aren't held against the subscription entity, so the charges billed aren't returned in the response.
    ///
    /// Once created, to get details of a one-time charge:
    /// - When created with `effective_from` as `next_billing_period`, get the subscription the charge was billed to and use the `include` query parameter with the `next_transaction` value.
    /// - When created with `effective_from` as `immediately`, list transactions and use the `subscription_id` query parameter with the subscription ID of the subscription the charge was billed to.
    ///
    /// When an update results in an immediate charge, responses may take longer than usual while a payment attempt is processed.
    pub fn subscription_one_time_charge(
        &self,
        subscription_id: impl Into<SubscriptionID>,
    ) -> subscriptions::SubscriptionOneTimeCharge<'_> {
        subscriptions::SubscriptionOneTimeCharge::new(self, subscription_id)
    }

    /// Activates a trialing subscription using its ID. Only automatically-collected subscriptions where the status is trialing can be activated.
    ///
    /// On activation, Paddle bills for a subscription immediately. Subscription billing dates are recalculated based on the activation date (the time the activation request is made).
    ///
    /// If successful, Paddle returns a copy of the updated subscription entity. The subscription status is `active`, and billing dates are updated to reflect the activation date.
    ///
    /// This operation results in an immediate charge, so responses may take longer than usual while a payment attempt is processed.
    pub async fn subscription_activate(
        &self,
        subscription_id: impl Into<SubscriptionID>,
    ) -> Result<Subscription> {
        let subscription_id = subscription_id.into();

        let url = format!("/subscriptions/{}/activate", subscription_id.as_ref());

        self.send(serde_json::json!({}), Method::POST, &url).await
    }

    /// Get a request builder for pausing a subscription using its ID.
    ///
    /// By default, subscriptions are paused at the end of the billing period. When you send a request to pause, Paddle creates a `scheduled_change` against the subscription entity to say that it should pause at the end of the current billing period. Its `status` remains `active` until after the effective date of the scheduled change, at which point it changes to `paused`.
    ///
    /// To set a resume date, include the `resume_at` field in your request. The subscription remains paused until the resume date, or until you send a resume request. Omit to create an open-ended pause. The subscription remains paused indefinitely, until you send a resume request.
    pub fn subscription_pause(
        &self,
        subscription_id: impl Into<SubscriptionID>,
    ) -> subscriptions::SubscriptionPause<'_> {
        subscriptions::SubscriptionPause::new(self, subscription_id)
    }

    /// Resumes a paused subscription using its ID. Only `paused` subscriptions can be resumed. If an `active` subscription has a scheduled change to pause in the future, use this operation to set or change the resume date.
    ///
    /// You can't resume a `canceled` subscription.
    ///
    /// On resume, Paddle bills for a subscription immediately by default. Subscription billing dates are recalculated based on the resume date. Use the `on_resume` field to change this behavior.
    ///
    /// If successful, Paddle returns a copy of the updated subscription entity:
    /// - When resuming a `paused` subscription immediately, the subscription status is `active`, and billing dates are updated to reflect the resume date.
    /// - When scheduling a `paused` subscription to resume on a date in the future, the subscription status is `paused`, and `scheduled_change.resume_at` is updated to reflect the scheduled resume date.
    /// - When changing the resume date for an `active` subscription that's scheduled to pause, the subscription status is `active` and `scheduled_change.resume_at` is updated to reflect the scheduled resume date.
    ///
    /// This operation may result in an immediate charge, so responses may take longer than usual while a payment attempt is processed.
    pub fn subscription_resume(
        &self,
        subscription_id: impl Into<SubscriptionID>,
    ) -> subscriptions::SubscriptionResume<'_> {
        subscriptions::SubscriptionResume::new(self, subscription_id)
    }

    /// Get a request builder for canceling a subscription.
    ///
    /// By default, active subscriptions are canceled at the end of the billing period. When you send a request to cancel, Paddle creates a `scheduled_change` against the subscription entity to say that it should cancel at the end of the current billing period. Its `status` remains `active` until after the effective date of the scheduled change, at which point it changes to `canceled`.
    ///
    /// You can cancel a subscription right away by including `effective_from` in your request, setting the value to `immediately`. If successful, your response includes a copy of the updated subscription entity with the `status` of `canceled`. Canceling immediately is the default behavior for paused subscriptions.
    ///
    /// You can't reinstate a canceled subscription.
    pub fn subscription_cancel(
        &self,
        subscription_id: impl Into<SubscriptionID>,
    ) -> subscriptions::SubscriptionCancel<'_> {
        subscriptions::SubscriptionCancel::new(self, subscription_id)
    }

    /// Get a request builder for retrieving adjustments from Paddle.
    ///
    /// Use the builder parameters to filter and page through results.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let res = client.adjustments_list().send().await.unwrap();
    /// dbg!(res.data);
    /// ```
    pub fn adjustments_list(&self) -> adjustments::AdjustmentsList<'_> {
        adjustments::AdjustmentsList::new(self)
    }

    /// Get a request builder for creating an adjustment for one or more transaction items.
    ///
    /// You can create adjustments to refund or credit all or part of a transaction and its items:
    /// - Refunds return an amount to a customer's original payment method. You can create refund adjustments for transactions that are `completed`.
    /// - Credits reduce the amount that a customer has to pay for a transaction. You can create credit adjustments for manually-collected transactions that are `billed` or `past_due`.
    ///
    /// You can create adjustments to refund transactions that are `completed`, or to reduce the amount to due on manually-collected transactions that are `billed` or `past_due`. Most refunds for live accounts are created with the status of `pending_approval` until reviewed by Paddle, but some are automatically approved. For sandbox accounts, Paddle automatically approves refunds every ten minutes.
    ///
    /// Adjustments can apply to some or all items on a transaction. You'll need the Paddle ID of the transaction to create a refund or credit for, along with the Paddle ID of any transaction items `(details.line_items[].id)`.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::{
    ///     enums::{AdjustmentAction, AdjustmentType},
    ///     Paddle,
    /// };
    ///
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    ///
    /// let res = client.adjustment_create("txn_01jkfx8v9z4pee0p5bd35x95bp", AdjustmentAction::Refund, "Refund reason")
    ///     .r#type(AdjustmentType::Full)
    ///     .send()
    ///     .await
    ///     .unwrap();
    ///
    /// dbg!(res.data);
    /// ```
    pub fn adjustment_create(
        &self,
        transaction_id: impl Into<TransactionID>,
        action: AdjustmentAction,
        reason: impl Into<String>,
    ) -> adjustments::AdjustmentCreate<'_> {
        adjustments::AdjustmentCreate::new(self, transaction_id, action, reason)
    }

    /// Returns a link to a credit note PDF for an adjustment.
    ///
    /// Credit note PDFs are created for refunds and credits as a record of an adjustment.
    ///
    /// The link returned is not a permanent link. It expires after an hour.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::{enums::Disposition, Paddle};
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let res = client.adjustment_credit_note("txn_01hv8wptq8987qeep44cyrewp9", Disposition::Inline).await.unwrap();
    /// dbg!(res.data.url)
    /// ```
    pub async fn adjustment_credit_note(
        &self,
        adjustment_id: impl Into<AdjustmentID>,
        disposition: Disposition,
    ) -> Result<TransactionInvoice> {
        let adjustment_id = adjustment_id.into();

        let url = format!("/adjustments/{}/credit-note", adjustment_id.as_ref());
        let params = ("disposition", disposition);

        self.send(params, Method::GET, &url).await
    }

    /// Get a request builder for fetching pricing previews for one or more prices. Typically used for building pricing pages.
    ///
    /// You can provide location information when previewing prices. You must provide this if you want Paddle to calculate tax or automatically localize prices. You can provide one of:
    /// - `customer_ip_address`: Paddle fetches location using the IP address to calculate totals.
    /// - `address`: Paddle uses the country and ZIP code (where supplied) to calculate totals.
    /// - `customer_id`, `address_id`, `business_id`: Paddle uses existing customer data to calculate totals. Typically used for logged-in customers.
    ///
    /// If successful, your response includes the data you sent with a details object that includes totals for the supplied prices.
    ///
    /// Each line item includes `formatted_unit_totals` and `formatted_totals` objects that return totals formatted for the country or region you're working with, including the currency symbol.
    ///
    /// You can work with the preview prices operation using the Paddle.PricePreview() method in Paddle.js. When working with Paddle.PricePreview(), request and response fields are camelCase rather than snake_case.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::{enums::Disposition, Paddle};
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    ///
    /// let res = client.pricing_preview()
    ///     .send([PricePreviewItem { price_id: "pri_01jqxvdyjkp961jzv4me7ezg4d".into(), quantity: 1, }])
    ///     .await
    ///     .unwrap();
    ///
    /// dbg!(res.data)
    /// ```
    pub fn pricing_preview(
        &self,
        items: impl IntoIterator<Item = PricePreviewItem>,
    ) -> pricing_preview::PricingPreview<'_> {
        pricing_preview::PricingPreview::new(self, items)
    }

    /// Get a request builder for fetching a single report in Paddle.
    pub fn reports_list<'a>(&'a self) -> reports::ReportsList<'a> {
        reports::ReportsList::new(self)
    }

    /// Returns a report using its ID.
    pub async fn report_get(&self, report_id: impl Into<PaddleID>) -> Result<ReportBase> {
        let report_id = report_id.into();

        let url = format!("/reports/{}", report_id.as_ref());

        self.send((), Method::GET, &url).await
    }

    /// Returns a link to a CSV file for a report.
    ///
    /// Only returned for reports that are ready. This means Paddle has completed processing the report and it's ready to download.
    ///
    /// The link returned is not a permanent link. It expires after 3 minutes.
    pub async fn report_download_url(
        &self,
        report_id: impl Into<PaddleID>,
    ) -> Result<TransactionInvoice> {
        let report_id = report_id.into();

        let url = format!("/reports/{}/download-url", report_id.as_ref());

        self.send((), Method::GET, &url).await
    }

    /// Get a request builder for creating reports in Paddle.
    ///
    /// Reports are created as `pending` initially while Paddle generates your report. They move to `ready` when they're ready to download.
    ///
    /// You can download a report when it's ready using the get a CSV file for a report operation.
    ///
    /// If successful, your response includes a copy of the new report entity.
    pub fn report_create<'a, T: ReportType + DeserializeOwned>(
        &'a self,
        report_type: T,
    ) -> reports::ReportCreate<'a, T> {
        reports::ReportCreate::new(self, report_type)
    }

    /// Returns a list of event types.
    ///
    /// The response is not paginated.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::{enums::Disposition, Paddle};
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let res = client.event_types_list().await.unwrap();
    /// dbg!(res.data)
    /// ```
    pub async fn event_types_list(&self) -> Result<Vec<EventType>> {
        self.send((), Method::GET, "/event-types").await
    }

    /// Returns a list of event types.
    ///
    /// The response is not paginated.
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// use paddle_rust_sdk::{enums::Disposition, Paddle};
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let res = client.events_list().send().await.unwrap();
    /// dbg!(res.data)
    /// ```
    pub fn events_list(&self) -> events::EventsList<'_> {
        events::EventsList::new(self)
    }

    async fn send<T: DeserializeOwned>(
        &self,
        req: impl Serialize,
        method: Method,
        path: &str,
    ) -> Result<T> {
        let mut url = self.base_url.join(path)?;
        let client = reqwest::Client::new();

        if method == reqwest::Method::GET {
            url.set_query(Some(&serde_qs::to_string(&req)?));
        }

        let mut builder = client
            .request(method.clone(), url)
            .bearer_auth(self.api_key.clone())
            .header(CONTENT_TYPE, "application/json; charset=utf-8");

        builder = match method {
            reqwest::Method::POST | reqwest::Method::PUT | reqwest::Method::PATCH => {
                builder.json(&req)
            }
            _ => builder,
        };

        // Uncomment this to see the raw text response
        // let text = builder.send().await?.text().await?;
        // println!("{}", text);
        // todo!();

        // Uncomment this to attempt to deserialize the response into an entity
        // Needed due to https://github.com/serde-rs/serde/issues/2157

        // let res: serde_json::Value = builder.send().await?.json().await?;
        // let data_json = serde_json::to_string(&res["data"]).unwrap();
        // let res: Vec<entities::ReportBase> = serde_json::from_str(&data_json).unwrap();
        // // println!("{}", serde_json::to_string(&res["data"]).unwrap());
        // todo!();

        let res: Response<_> = builder.send().await?.json().await?;

        match res {
            Response::Success(success) => Ok(success),
            Response::Error(error) => Err(Error::PaddleApi(error)),
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

fn comma_separated_enum<S, T>(
    values: &Option<Vec<T>>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    T: Serialize,
{
    match values {
        Some(values) => {
            let mut serialized = vec![];

            for val in values {
                let serde_value = serde_json::to_value(val).map_err(serde::ser::Error::custom)?;
                let serialized_value = serde_value
                    .as_str()
                    .ok_or(serde::ser::Error::custom("Failed to serialize enum"))?
                    .to_string();

                serialized.push(serialized_value);
            }

            serializer.serialize_str(serialized.join(",").as_str())
        }
        None => serializer.serialize_none(),
    }
}
