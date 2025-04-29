//! Builders for making requests to the Paddle API for customer saved payment methods.
//!
//! See the [Paddle API](https://developer.paddle.com/api-reference/payment-methods/overview) documentation for more information.

use reqwest::Method;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::entities::PaymentMethod;
use crate::ids::{AddressID, CustomerID, PaymentMethodID};
use crate::{Paddle, Result};

/// Request builder for fetching businesses from Paddle API.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct PaymentMethodsList<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    #[serde(skip)]
    customer_id: CustomerID,
    #[serde(serialize_with = "crate::comma_separated")]
    address_id: Option<Vec<AddressID>>,
    after: Option<PaymentMethodID>,
    order_by: Option<String>,
    per_page: Option<usize>,
    supports_checkout: Option<bool>,
}

impl<'a> PaymentMethodsList<'a> {
    pub fn new(client: &'a Paddle, customer_id: impl Into<CustomerID>) -> Self {
        Self {
            client,
            customer_id: customer_id.into(),
            after: None,
            address_id: None,
            order_by: None,
            per_page: None,
            supports_checkout: None,
        }
    }

    /// Return entities related to the specified addresses.
    pub fn address_ids(
        &mut self,
        address_ids: impl IntoIterator<Item = impl Into<AddressID>>,
    ) -> &mut Self {
        self.address_id = Some(address_ids.into_iter().map(Into::into).collect());
        self
    }

    /// Return entities after the specified Paddle ID when working with paginated endpoints. Used in the `meta.pagination.next` URL in responses for list operations.
    pub fn after(&mut self, id: impl Into<PaymentMethodID>) -> &mut Self {
        self.after = Some(id.into());
        self
    }

    /// Order returned entities by the specified field. Valid fields for ordering: id
    pub fn order_by_asc(&mut self, field: &str) -> &mut Self {
        self.order_by = Some(format!("{}[ASC]", field));
        self
    }

    /// Order returned entities by the specified field. Valid fields for ordering: id
    pub fn order_by_desc(&mut self, field: &str) -> &mut Self {
        self.order_by = Some(format!("{}[DESC]", field));
        self
    }

    /// Set how many entities are returned per page. Paddle returns the maximum number of results if a number greater than the maximum is requested.
    /// Check `meta.pagination.per_page` in the response to see how many were returned.
    ///
    /// Default: `50`; Maximum: `200`.
    pub fn per_page(&mut self, entities_per_page: usize) -> &mut Self {
        self.per_page = Some(entities_per_page);
        self
    }

    /// Return entities that support being presented at checkout (`true`) or not (`false`).
    pub fn supports_checkout(&mut self, flag: bool) -> &mut Self {
        self.supports_checkout = Some(flag);
        self
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<Vec<PaymentMethod>> {
        self.client
            .send(
                self,
                Method::GET,
                &format!("/customers/{}/payment-methods", self.customer_id.as_ref()),
            )
            .await
    }
}

/// Request builder for fetching a single payment method from Paddle API.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct PaymentMethodGet<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    #[serde(skip)]
    customer_id: CustomerID,
    #[serde(skip)]
    payment_method_id: PaymentMethodID,
}

impl<'a> PaymentMethodGet<'a> {
    pub fn new(
        client: &'a Paddle,
        customer_id: impl Into<CustomerID>,
        payment_method_id: impl Into<PaymentMethodID>,
    ) -> Self {
        Self {
            client,
            customer_id: customer_id.into(),
            payment_method_id: payment_method_id.into(),
        }
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<PaymentMethod> {
        self.client
            .send(
                self,
                Method::GET,
                &format!(
                    "/customers/{}/payment-methods/{}",
                    self.customer_id.as_ref(),
                    self.payment_method_id.as_ref()
                ),
            )
            .await
    }
}
