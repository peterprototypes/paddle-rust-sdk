//! Builders for making requests to the Paddle API for customer businesses.
//!
//! See the [Paddle API](https://developer.paddle.com/api-reference/businesses/overview) documentation for more information.

use std::collections::HashMap;

use reqwest::Method;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::entities::{Business, Contact};
use crate::enums::Status;
use crate::ids::CustomerID;
use crate::{Paddle, Result};

/// Request builder for creating customer businesses in Paddle API.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct BusinessCreate<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    #[serde(skip)]
    customer_id: CustomerID,
    name: String,
    company_number: Option<String>,
    tax_identifier: Option<String>,
    contacts: Option<Vec<Contact>>,
    custom_data: Option<HashMap<String, String>>,
}

impl<'a> BusinessCreate<'a> {
    pub fn new(
        client: &'a Paddle,
        customer_id: impl Into<CustomerID>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            client,
            customer_id: customer_id.into(),
            name: name.into(),
            company_number: None,
            tax_identifier: None,
            contacts: None,
            custom_data: None,
        }
    }

    /// Company number for this business.
    pub fn company_number(&mut self, company_number: impl Into<String>) -> &mut Self {
        self.company_number = Some(company_number.into());
        self
    }

    /// Tax identifier for this business.
    pub fn tax_identifier(&mut self, tax_identifier: impl Into<String>) -> &mut Self {
        self.tax_identifier = Some(tax_identifier.into());
        self
    }

    /// Contact information for this business.
    pub fn contacts(&mut self, contacts: impl IntoIterator<Item = Contact>) -> &mut Self {
        self.contacts = Some(contacts.into_iter().collect());
        self
    }

    /// Custom data for this business.
    pub fn custom_data(&mut self, custom_data: HashMap<String, String>) -> &mut Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<Business> {
        self.client
            .send(
                self,
                Method::POST,
                &format!("/customers/{}/businesses", self.customer_id.as_ref()),
            )
            .await
    }
}
