//! # Paddle API Client
//!
//! This is a Rust client for the Paddle API, which allows you to interact with Paddle's services.

use std::fmt::Display;

use entities::{CustomerAuthenticationToken, TransactionInvoice};
use reqwest::{header::CONTENT_TYPE, IntoUrl, Method, StatusCode, Url};
use serde::{de::DeserializeOwned, Serialize};

pub mod entities;
pub mod enums;
pub mod error;
pub mod ids;

pub mod addresses;
pub mod businesses;
pub mod customers;
pub mod discounts;
pub mod payment_methods;
pub mod prices;
pub mod products;
pub mod subscriptions;
pub mod transactions;

pub mod response;

use enums::{CountryCodeSupported, CurrencyCode, DiscountType, Disposition, TaxCategory};
use ids::{
    AddressID, BusinessID, CustomerID, DiscountID, PaymentMethodID, PriceID, ProductID,
    SubscriptionID, TransactionID,
};

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
    /// use paddle_rust_sdk::Paddle;
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
    /// use paddle_rust_sdk::Paddle;
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
    /// use paddle_rust_sdk::Paddle;
    /// use paddle_rust_sdk::enums::TaxCategory;
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
    /// use paddle_rust_sdk::Paddle;
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
    /// use paddle_rust_sdk::Paddle;
    /// use paddle_rust_sdk::enums::TaxCategory;
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
    /// use paddle_rust_sdk::Paddle;
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
    ) -> prices::PricesCreate {
        prices::PricesCreate::new(self, product_id, description, amount, currency)
    }

    /// Returns a request builder for fetching a specific price by id.
    ///
    /// # Example:
    /// ```
    /// use paddle_rust_sdk::Paddle;
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
    /// use paddle_rust_sdk::Paddle;
    /// use paddle_rust_sdk::enums::TaxCategory;
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
    /// use paddle_rust_sdk::Paddle;
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
    /// use paddle_rust_sdk::Paddle;
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
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let discount = client.discount_get("dsc_01jqzpbmnq...").send().await.unwrap();
    /// ```
    pub fn discount_get(&self, discount_id: impl Into<DiscountID>) -> discounts::DiscountGet {
        discounts::DiscountGet::new(self, discount_id)
    }

    /// Returns a request builder for creating discounts.
    ///
    /// # Example:
    /// ```
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let discount = client.discount_update("dsc_01jqzpbmnq...").amount("18").send().await.unwrap();
    /// ```
    pub fn discount_update(&self, discount_id: impl Into<DiscountID>) -> discounts::DiscountUpdate {
        discounts::DiscountUpdate::new(self, discount_id)
    }

    /// Returns a request builder for fetching customers. Use the after method to page through results.
    ///
    /// By default, Paddle returns customers that are `active`. Use the status query parameter to return customers that are archived.
    ///
    /// # Example:
    /// ```
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let customers = client.customers_list().send().await.unwrap();
    /// ```
    pub fn customers_list(&self) -> customers::CustomersList {
        customers::CustomersList::new(self)
    }

    /// Returns a request builder for creating a new customer.
    ///
    /// # Example:
    /// ```
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let customers = client.customer_create("test@example.com").send().await.unwrap();
    /// ```
    pub fn customer_create(&self, email: impl Into<String>) -> customers::CustomerCreate {
        customers::CustomerCreate::new(self, email.into())
    }

    /// Returns a request builder for fetching a specific customer by id.
    ///
    /// # Example:
    /// ```
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let discount = client.customer_get("ctm_01jqztc78e1xfdgwhcgjzdrvgd").send().await.unwrap();
    /// ```
    pub fn customer_get(&self, customer_id: impl Into<CustomerID>) -> customers::CustomerGet {
        customers::CustomerGet::new(self, customer_id)
    }

    /// Returns a request builder for updating customer data.
    ///
    /// # Example:
    /// ```
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let discount = client.customer_update("ctm_01jqztc78e1xfdgwhcgjzdrvgd").email("new_email@example.com").send().await.unwrap();
    /// ```
    pub fn customer_update(&self, customer_id: impl Into<CustomerID>) -> customers::CustomerUpdate {
        customers::CustomerUpdate::new(self, customer_id)
    }

    /// Returns a request builder for fetching a list of credit balances for each currency for a customer.
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
    /// ```
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let discount = client.customer_credit_balances("ctm_01jqztc78e1xfdgwhcgjzdrvgd").send().await.unwrap();
    /// ```
    pub fn customer_credit_balances(
        &self,
        customer_id: impl Into<CustomerID>,
    ) -> customers::CustomerCreditBalances {
        customers::CustomerCreditBalances::new(self, customer_id)
    }

    /// Generates an authentication token for a customer.
    ///
    /// You can pass a generated authentication token to Paddle.js when opening a checkout to let customers work with saved payment methods.
    ///
    /// Authentication tokens are temporary and shouldn't be cached. They're valid until the expires_at date returned in the response.
    pub async fn generate_auth_token(
        &self,
        customer_id: impl Display,
    ) -> Result<CustomerAuthenticationToken> {
        let client = reqwest::Client::new();

        let url = format!("{}customers/{}/auth-token", self.base_url, customer_id);

        let res: Response<_> = client
            .post(url)
            .bearer_auth(self.api_key.clone())
            .send()
            .await?
            .json()
            .await?;

        match res {
            Response::Success(success) => Ok(success),
            Response::Error(error) => Err(Error::Paddle(error)),
        }
    }

    /// Returns a request builder for fetching customers addresses.
    ///
    /// By default, Paddle returns addresses that are `active`. Use the status query parameter to return addresses that are archived.
    ///
    /// # Example:
    /// ```
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let customers = client.addresses_list("ctm_01jqztc78e1xfdgwhcgjzdrvgd").send().await.unwrap();
    /// ```
    pub fn addresses_list(&self, customer_id: impl Into<CustomerID>) -> addresses::AddressesList {
        addresses::AddressesList::new(self, customer_id)
    }

    /// Returns a request builder for creating a new customer address.
    ///
    /// # Example:
    /// ```
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let customers = client.address_create("ctm_01jqztc78e1xfdgwhcgjzdrvgd", CountryCodeSupported::US).send().await.unwrap();
    /// ```
    pub fn address_create(
        &self,
        customer_id: impl Into<CustomerID>,
        country_code: CountryCodeSupported,
    ) -> addresses::AddressCreate {
        addresses::AddressCreate::new(self, customer_id, country_code)
    }

    /// Returns a request builder for getting an address for a customer using its ID and related customer ID.
    ///
    /// # Example:
    /// ```
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let customers = client.address_get("ctm_01jqztc78e1xfdgwhcgjzdrvgd", "add_01hv8gwdfkw5z6d1yy6pa3xyrz").send().await.unwrap();
    /// ```
    pub fn address_get(
        &self,
        customer_id: impl Into<CustomerID>,
        address_id: impl Into<AddressID>,
    ) -> addresses::AddressGet {
        addresses::AddressGet::new(self, customer_id, address_id)
    }

    /// Returns a request builder for updating an address for a customer using its ID and related customer ID.
    ///
    /// # Example:
    /// ```
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let customers = client.address_update("ctm_01jqztc78e1xfdgwhcgjzdrvgd", "add_01hv8gwdfkw5z6d1yy6pa3xyrz").first_line("Test").send().await.unwrap();
    /// ```
    pub fn address_update(
        &self,
        customer_id: impl Into<CustomerID>,
        address_id: impl Into<AddressID>,
    ) -> addresses::AddressUpdate {
        addresses::AddressUpdate::new(self, customer_id, address_id)
    }

    /// Returns a request builder for fetching customers businesses.
    ///
    /// By default, Paddle returns addresses that are `active`. Use the status query parameter to return businesses that are archived.
    ///
    /// # Example:
    /// ```
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let customers = client.businesses_list("ctm_01jqztc78e1xfdgwhcgjzdrvgd").send().await.unwrap();
    /// ```
    pub fn businesses_list(
        &self,
        customer_id: impl Into<CustomerID>,
    ) -> businesses::BusinessesList {
        businesses::BusinessesList::new(self, customer_id)
    }

    /// Returns a request builder for creating a new customer business.
    ///
    /// # Example:
    /// ```
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let customers = client.business_create("ctm_01jqztc78e1xfdgwhcgjzdrvgd", "Company Inc.").send().await.unwrap();
    /// ```
    pub fn business_create(
        &self,
        customer_id: impl Into<CustomerID>,
        name: impl Into<String>,
    ) -> businesses::BusinessCreate {
        businesses::BusinessCreate::new(self, customer_id, name)
    }

    /// Returns a request builder for getting a business for a customer using its ID and related customer ID.
    ///
    /// # Example:
    /// ```
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let customers = client.business_get("ctm_01jqztc78e1xfdgwhcgjzdrvgd", "biz_01jr85bypq4d3w139m53zw2559").send().await.unwrap();
    /// ```
    pub fn business_get(
        &self,
        customer_id: impl Into<CustomerID>,
        business_id: impl Into<BusinessID>,
    ) -> businesses::BusinessGet {
        businesses::BusinessGet::new(self, customer_id, business_id)
    }

    /// Returns a request builder for updating a business for a customer using its ID and related customer ID.
    ///
    /// # Example:
    /// ```
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let customers = client.business_update("ctm_01jqztc78e1xfdgwhcgjzdrvgd", "biz_01jr85bypq4d3w139m53zw2559").first_line("Test").send().await.unwrap();
    /// ```
    pub fn business_update(
        &self,
        customer_id: impl Into<CustomerID>,
        business_id: impl Into<BusinessID>,
    ) -> businesses::BusinessUpdate {
        businesses::BusinessUpdate::new(self, customer_id, business_id)
    }

    /// Returns a request builder for querying customer saved payment methods.
    ///
    /// # Example:
    /// ```
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let customers = client.payment_methods_list("ctm_01jqztc78e1xfdgwhcgjzdrvgd").send().await.unwrap();
    /// ```
    pub fn payment_methods_list(
        &self,
        customer_id: impl Into<CustomerID>,
    ) -> payment_methods::PaymentMethodsList {
        payment_methods::PaymentMethodsList::new(self, customer_id)
    }

    /// Returns a request builder for getting a saved payment for a customer using its ID and related customer ID.
    ///
    /// # Example:
    /// ```
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let customers = client.payment_method_get("ctm_01jqztc78e1xfdgwhcgjzdrvgd", "paymtd_01j2jff1m3es31sdkejpaym164").send().await.unwrap();
    /// ```
    pub fn payment_method_get(
        &self,
        customer_id: impl Into<CustomerID>,
        payment_method_id: impl Into<PaymentMethodID>,
    ) -> payment_methods::PaymentMethodGet {
        payment_methods::PaymentMethodGet::new(self, customer_id, payment_method_id)
    }

    /// Deletes a customer payment method using its ID.
    ///
    /// When you delete a customer payment method, it's permanently removed from that customer.
    ///
    /// There's no way to recover a deleted payment method.
    ///
    /// # Example:
    /// ```
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
    /// ```
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let session = client.create_portal_session("ctm_01jqztc78e1xfdgwhcgjzdrvgd").send().await.unwrap();
    /// dbg!(session.data.urls.general.overview);
    /// dbg!(session.data.urls.subscriptions);
    /// ```
    pub fn create_portal_session(
        &self,
        customer_id: impl Into<CustomerID>,
    ) -> customers::PortalSessionCreate {
        customers::PortalSessionCreate::new(self, customer_id)
    }

    /// Returns a request builder for querying transactions.
    ///
    /// Use the include method on the builder to include related entities in the response.
    ///
    /// # Example:
    /// ```
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let transactions = client.transactions_list().send().await.unwrap();
    /// ```
    pub fn transactions_list(&self) -> transactions::TransactionsList {
        transactions::TransactionsList::new(self)
    }

    /// Returns a request builder for creating a transaction.
    ///
    /// See [Create Transaction](https://developer.paddle.com/api-reference/transactions/create-transaction) for more information.
    ///
    /// # Example:
    /// ```
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
    pub fn transaction_create(&self) -> transactions::TransactionCreate {
        transactions::TransactionCreate::new(self)
    }

    /// Returns a request builder for fetching a transaction using its ID.
    ///
    /// # Example:
    /// ```
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let res = client.transaction_get("txn_01hv8wptq8987qeep44cyrewp9").send().await.unwrap();
    /// dbg!(res.data);
    /// ```
    pub fn transaction_get(
        &self,
        transaction_id: impl Into<TransactionID>,
    ) -> transactions::TransactionGet {
        transactions::TransactionGet::new(self, transaction_id)
    }

    /// Returns a request builder for updating a transaction.
    ///
    /// # Example:
    /// ```
    /// use paddle_rust_sdk::{enums::TransactionStatus, Paddle};
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// client.transaction_update("txn_01hv8wptq8987qeep44cyrewp9").status(TransactionStatus::Billed).send().await.unwrap();
    /// ```
    pub fn transaction_update(
        &self,
        transaction_id: impl Into<TransactionID>,
    ) -> transactions::TransactionUpdate {
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
    /// ```
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

    /// Returns a request builder for generating a transaction preview without creating a transaction entity. Typically used for creating more advanced, dynamic pricing pages where users can build their own plans.
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
    pub fn transaction_preview(&self) -> transactions::TransactionPreview {
        transactions::TransactionPreview::new(self)
    }

    /// Returns a request builder to revise customer information for a billed or completed transaction.
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
    ) -> transactions::TransactionRevise {
        transactions::TransactionRevise::new(self, transaction_id)
    }

    /// Returns a request builder for querying subscriptions.
    ///
    /// # Example:
    /// ```
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let subscriptions = client.subscriptions_list().send().await.unwrap();
    /// ```
    pub fn subscriptions_list(&self) -> subscriptions::SubscriptionsList {
        subscriptions::SubscriptionsList::new(self)
    }

    /// Returns a request builder for fetching a subscription using its ID.
    ///
    /// # Example:
    /// ```
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let res = client.subscription_get("sub_01hv8y5ehszzq0yv20ttx3166y").send().await.unwrap();
    /// dbg!(res.data);
    /// ```
    pub fn subscription_get(
        &self,
        subscription_id: impl Into<SubscriptionID>,
    ) -> subscriptions::SubscriptionGet {
        subscriptions::SubscriptionGet::new(self, subscription_id)
    }

    /// Returns a request builder for getting a preview of changes to a subscription without actually applying them.
    ///
    /// Typically used for previewing proration before making changes to a subscription.
    ///
    /// If successful, your response includes `immediate_transaction`, `next_transaction`, and `recurring_transaction_details` so you can see expected transactions for the changes.
    ///
    /// The `update_summary` object contains details of prorated credits and charges created, along with the overall result of the update.
    ///
    /// /// # Example:
    /// ```
    /// use paddle_rust_sdk::Paddle;
    /// let client = Paddle::new("your_api_key", Paddle::SANDBOX).unwrap();
    /// let res = client.subscription_get("sub_01hv8y5ehszzq0yv20ttx3166y").send().await.unwrap();
    /// dbg!(res.data);
    /// ```
    pub fn subscription_preview_update(
        &self,
        subscription_id: impl Into<SubscriptionID>,
    ) -> subscriptions::SubscriptionPreviewUpdate {
        subscriptions::SubscriptionPreviewUpdate::new(self, subscription_id)
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
            dbg!(&serde_qs::to_string(&req)?);
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
        // let res: entities::SubscriptionPreview = serde_json::from_str(&data_json).unwrap();
        // // println!("{}", serde_json::to_string(&res["data"]).unwrap());
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
