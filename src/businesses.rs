//! Builders for making requests to the Paddle API for customer businesses.
//!
//! See the [Paddle API](https://developer.paddle.com/api-reference/businesses/overview) documentation for more information.

use std::collections::HashMap;

use reqwest::Method;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::entities::{Business, Contact};
use crate::enums::Status;
use crate::ids::{BusinessID, CustomerID};
use crate::{Paddle, Result};

/// Request builder for fetching businesses from Paddle API.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct BusinessesList<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    #[serde(skip)]
    customer_id: CustomerID,
    after: Option<BusinessID>,
    #[serde(serialize_with = "crate::comma_separated")]
    id: Option<Vec<BusinessID>>,
    order_by: Option<String>,
    per_page: Option<usize>,
    search: Option<String>,
    status: Option<Status>,
}

impl<'a> BusinessesList<'a> {
    pub fn new(client: &'a Paddle, customer_id: impl Into<CustomerID>) -> Self {
        Self {
            client,
            customer_id: customer_id.into(),
            after: None,
            id: None,
            order_by: None,
            per_page: None,
            search: None,
            status: None,
        }
    }

    /// Return entities after the specified Paddle ID when working with paginated endpoints. Used in the `meta.pagination.next` URL in responses for list operations.
    pub fn after(&mut self, business_id: BusinessID) -> &mut Self {
        self.after = Some(business_id);
        self
    }

    /// Return only the IDs specified.
    pub fn ids(
        &mut self,
        business_ids: impl IntoIterator<Item = impl Into<BusinessID>>,
    ) -> &mut Self {
        self.id = Some(business_ids.into_iter().map(Into::into).collect());
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

    /// Return entities that match a search query. Searches `status`, `created_at`, and `updated_at`.
    pub fn search(&mut self, term: impl Into<String>) -> &mut Self {
        self.search = Some(term.into());
        self
    }

    /// Return only prices with the specified status.
    pub fn status(&mut self, status: Status) -> &mut Self {
        self.status = Some(status);
        self
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<Vec<Business>> {
        self.client
            .send(
                self,
                Method::GET,
                &format!("/customers/{}/businesses", self.customer_id.as_ref()),
            )
            .await
    }
}

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
