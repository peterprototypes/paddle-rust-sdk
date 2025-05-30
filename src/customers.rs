//! Builders for making requests to the Paddle API for customers.
//!
//! See the [Paddle API](https://developer.paddle.com/api-reference/customers/overview) documentation for more information.

use std::collections::HashMap;

use reqwest::Method;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::entities::{CreditBalance, Customer, CustomerPortalSession};
use crate::enums::Status;
use crate::ids::{CustomerID, SubscriptionID};
use crate::paginated::Paginated;
use crate::{Paddle, Result};

/// Request builder for fetching customers from Paddle API.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct CustomersList<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    after: Option<CustomerID>,
    #[serde(serialize_with = "crate::comma_separated")]
    email: Option<Vec<String>>,
    #[serde(serialize_with = "crate::comma_separated")]
    id: Option<Vec<CustomerID>>,
    order_by: Option<String>,
    per_page: Option<usize>,
    search: Option<String>,
    status: Option<Status>,
}

impl<'a> CustomersList<'a> {
    pub fn new(client: &'a Paddle) -> Self {
        Self {
            client,
            after: None,
            email: None,
            id: None,
            order_by: None,
            per_page: None,
            search: None,
            status: None,
        }
    }

    /// Return entities after the specified Paddle ID when working with paginated endpoints. Used in the `meta.pagination.next` URL in responses for list operations.
    pub fn after(&mut self, customer_id: impl Into<CustomerID>) -> &mut Self {
        self.after = Some(customer_id.into());
        self
    }

    /// Return entities that exactly match the specified email addresses
    pub fn emails(&mut self, emails: impl IntoIterator<Item = impl AsRef<str>>) -> &mut Self {
        self.email = Some(emails.into_iter().map(|s| s.as_ref().to_string()).collect());
        self
    }

    /// Return only the IDs specified.
    pub fn ids(
        &mut self,
        customer_ids: impl IntoIterator<Item = impl Into<CustomerID>>,
    ) -> &mut Self {
        self.id = Some(customer_ids.into_iter().map(Into::into).collect());
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

    /// Return entities that match a search query. Searches `id`, `name`, and `email` fields. Use the email filter for precise matching of email addresses.
    pub fn search(&mut self, term: impl Into<String>) -> &mut Self {
        self.search = Some(term.into());
        self
    }

    /// Return only prices with the specified status.
    pub fn status(&mut self, status: Status) -> &mut Self {
        self.status = Some(status);
        self
    }

    /// Returns a paginator for fetching pages of entities from Paddle
    pub fn send(&self) -> Paginated<Vec<Customer>> {
        Paginated::new(self.client, "/customers", self)
    }
}

/// Request builder for creating customers in Paddle API.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct CustomerCreate<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    email: String,
    name: Option<String>,
    custom_data: Option<HashMap<String, String>>,
    locale: Option<String>,
}

impl<'a> CustomerCreate<'a> {
    pub fn new(client: &'a Paddle, email: String) -> Self {
        Self {
            client,
            email,
            name: None,
            custom_data: None,
            locale: None,
        }
    }

    /// Full name of this customer. Required when creating transactions where `collection_mode` is `manual` (invoices).
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    /// Your own structured key-value data.
    pub fn custom_data(&mut self, custom_data: HashMap<String, String>) -> &mut Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Valid IETF BCP 47 short form locale tag. If omitted, defaults to `en`.
    pub fn locale(&mut self, locale: impl Into<String>) -> &mut Self {
        self.locale = Some(locale.into());
        self
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<Customer> {
        self.client.send(self, Method::POST, "/customers").await
    }
}

/// Request builder for fetching a single customer from Paddle API.
#[derive(Serialize)]
pub struct CustomerGet<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    #[serde(skip)]
    customer_id: CustomerID,
}

impl<'a> CustomerGet<'a> {
    pub fn new(client: &'a Paddle, customer_id: impl Into<CustomerID>) -> Self {
        Self {
            client,
            customer_id: customer_id.into(),
        }
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<Customer> {
        self.client
            .send(
                self,
                Method::GET,
                &format!("/customers/{}", self.customer_id.as_ref()),
            )
            .await
    }
}

/// Request builder for updating a customer in Paddle API.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct CustomerUpdate<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    #[serde(skip)]
    customer_id: CustomerID,
    name: Option<String>,
    email: Option<String>,
    status: Option<Status>,
    custom_data: Option<HashMap<String, String>>,
    locale: Option<String>,
}

impl<'a> CustomerUpdate<'a> {
    pub fn new(client: &'a Paddle, customer_id: impl Into<CustomerID>) -> Self {
        Self {
            client,
            customer_id: customer_id.into(),
            name: None,
            email: None,
            status: None,
            custom_data: None,
            locale: None,
        }
    }

    /// Full name of this customer. Required when creating transactions where `collection_mode` is `manual` (invoices).
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    /// Email address for this customer.
    pub fn email(&mut self, email: impl Into<String>) -> &mut Self {
        self.email = Some(email.into());
        self
    }

    /// Whether this entity can be used in Paddle.
    pub fn status(&mut self, status: Status) -> &mut Self {
        self.status = Some(status);
        self
    }

    /// Your own structured key-value data.
    pub fn custom_data(&mut self, custom_data: HashMap<String, String>) -> &mut Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Valid IETF BCP 47 short form locale tag.
    pub fn locale(&mut self, locale: impl Into<String>) -> &mut Self {
        self.locale = Some(locale.into());
        self
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<Customer> {
        self.client
            .send(
                self,
                Method::PATCH,
                &format!("/customers/{}", self.customer_id.as_ref()),
            )
            .await
    }
}

/// Request builder for retrieving credit balances for each currency for a customer.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct CustomerCreditBalances<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    #[serde(skip)]
    customer_id: CustomerID,
}

impl<'a> CustomerCreditBalances<'a> {
    pub fn new(client: &'a Paddle, customer_id: impl Into<CustomerID>) -> Self {
        Self {
            client,
            customer_id: customer_id.into(),
        }
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<Vec<CreditBalance>> {
        self.client
            .send(
                self,
                Method::GET,
                &format!("/customers/{}/credit-balances", self.customer_id.as_ref()),
            )
            .await
    }
}

/// Request builder for creating customer portal sessions
#[skip_serializing_none]
#[derive(Serialize)]
pub struct PortalSessionCreate<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    #[serde(skip)]
    customer_id: CustomerID,
    subscription_ids: Option<Vec<SubscriptionID>>,
}

impl<'a> PortalSessionCreate<'a> {
    pub fn new(client: &'a Paddle, customer_id: impl Into<CustomerID>) -> Self {
        Self {
            client,
            customer_id: customer_id.into(),
            subscription_ids: None,
        }
    }

    /// List of subscriptions to create authenticated customer portal deep links for.
    pub fn subscription_ids(
        &mut self,
        subscription_ids: impl IntoIterator<Item = impl Into<SubscriptionID>>,
    ) -> &mut Self {
        self.subscription_ids = Some(subscription_ids.into_iter().map(Into::into).collect());
        self
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<CustomerPortalSession> {
        self.client
            .send(
                self,
                Method::POST,
                &format!("/customers/{}/portal-sessions", self.customer_id.as_ref()),
            )
            .await
    }
}
