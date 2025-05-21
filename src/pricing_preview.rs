//! Builders for making requests to the Paddle API for previewing prices.
//!
//! See the [Paddle API](https://developer.paddle.com/api-reference/pricing-preview/overview) documentation for more information.

use reqwest::Method;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::entities::{self, AddressPreview, PricePreviewItem};
use crate::enums::CurrencyCode;
use crate::ids::{AddressID, BusinessID, CustomerID, DiscountID};
use crate::{Paddle, Result};

/// Request builder for fetching transactions from Paddle API.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct PricingPreview<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    items: Vec<PricePreviewItem>,
    customer_id: Option<CustomerID>,
    address_id: Option<AddressID>,
    business_id: Option<BusinessID>,
    currency_code: Option<CurrencyCode>,
    discount_id: Option<DiscountID>,
    address: Option<AddressPreview>,
    customer_ip_address: Option<String>,
}

impl<'a> PricingPreview<'a> {
    pub fn new(client: &'a Paddle, items: impl IntoIterator<Item = PricePreviewItem>) -> Self {
        Self {
            client,
            items: items.into_iter().collect(),
            customer_id: None,
            address_id: None,
            business_id: None,
            currency_code: None,
            discount_id: None,
            address: None,
            customer_ip_address: None,
        }
    }

    /// Paddle ID of the customer that this preview is for.
    pub fn customer_id(&mut self, customer_id: impl Into<CustomerID>) -> &mut Self {
        self.customer_id = Some(customer_id.into());
        self
    }

    /// Paddle ID of the address that this preview is for.
    ///
    /// Send one of `address_id`, `customer_ip_address`, or the `address` object when previewing.
    pub fn address_id(&mut self, address_id: impl Into<AddressID>) -> &mut Self {
        self.address_id = Some(address_id.into());
        self
    }

    /// Paddle ID of the business that this preview is for.
    pub fn business_id(&mut self, business_id: impl Into<BusinessID>) -> &mut Self {
        self.business_id = Some(business_id.into());
        self
    }

    /// Supported three-letter ISO 4217 currency code.
    pub fn currency_code(&mut self, currency_code: CurrencyCode) -> &mut Self {
        self.currency_code = Some(currency_code);
        self
    }

    /// Paddle ID of the discount applied to this preview
    pub fn discount_id(&mut self, discount_id: impl Into<DiscountID>) -> &mut Self {
        self.discount_id = Some(discount_id.into());
        self
    }

    /// Address for this preview. Send one of `address_id`, `customer_ip_address`, or the `address` object when previewing.
    pub fn address(&mut self, address: AddressPreview) -> &mut Self {
        self.address = Some(address);
        self
    }

    /// IP address for this transaction preview. Send one of `address_id`, `customer_ip_address`, or the `address` object when previewing.
    pub fn customer_ip_address(&mut self, ip: String) -> &mut Self {
        self.customer_ip_address = Some(ip);
        self
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<entities::PricingPreview> {
        self.client
            .send(self, Method::POST, "/pricing-preview")
            .await
    }
}
