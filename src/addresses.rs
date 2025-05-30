//! Builders for making requests to the Paddle API for customer addresses.
//!
//! See the [Paddle API](https://developer.paddle.com/api-reference/addresses/overview) documentation for more information.

use std::collections::HashMap;

use reqwest::Method;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::entities::Address;
use crate::enums::{CountryCodeSupported, Status};
use crate::ids::{AddressID, CustomerID};
use crate::paginated::Paginated;
use crate::{Paddle, Result};

/// Request builder for fetching addresses from Paddle API.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct AddressesList<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    #[serde(skip)]
    customer_id: CustomerID,
    after: Option<AddressID>,
    #[serde(serialize_with = "crate::comma_separated")]
    id: Option<Vec<AddressID>>,
    order_by: Option<String>,
    per_page: Option<usize>,
    search: Option<String>,
    status: Option<Status>,
}

impl<'a> AddressesList<'a> {
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
    pub fn after(&mut self, address_id: impl Into<AddressID>) -> &mut Self {
        self.after = Some(address_id.into());
        self
    }

    /// Return only the IDs specified.
    pub fn ids(
        &mut self,
        address_ids: impl IntoIterator<Item = impl Into<AddressID>>,
    ) -> &mut Self {
        self.id = Some(address_ids.into_iter().map(Into::into).collect());
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
    pub fn send(&self) -> Paginated<Vec<Address>> {
        let url = format!("/customers/{}/addresses", self.customer_id.as_ref());

        Paginated::new(self.client, &url, self)
    }
}

/// Request builder for creating customer addresses in Paddle API.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct AddressCreate<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    #[serde(skip)]
    customer_id: CustomerID,
    country_code: CountryCodeSupported,
    description: Option<String>,
    first_line: Option<String>,
    second_line: Option<String>,
    city: Option<String>,
    postal_code: Option<String>,
    region: Option<String>,
    custom_data: Option<HashMap<String, String>>,
}

impl<'a> AddressCreate<'a> {
    pub fn new(
        client: &'a Paddle,
        customer_id: impl Into<CustomerID>,
        country_code: CountryCodeSupported,
    ) -> Self {
        Self {
            client,
            customer_id: customer_id.into(),
            country_code,
            description: None,
            first_line: None,
            second_line: None,
            city: None,
            postal_code: None,
            region: None,
            custom_data: None,
        }
    }

    /// Memorable description for this address.
    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        self.description = Some(description.into());
        self
    }

    /// First line of the address.
    pub fn first_line(&mut self, first_line: impl Into<String>) -> &mut Self {
        self.first_line = Some(first_line.into());
        self
    }

    /// Second line of the address.
    pub fn second_line(&mut self, second_line: impl Into<String>) -> &mut Self {
        self.second_line = Some(second_line.into());
        self
    }

    /// City name.
    pub fn city(&mut self, city: impl Into<String>) -> &mut Self {
        self.city = Some(city.into());
        self
    }

    /// Postal code. Required for US addresses
    pub fn postal_code(&mut self, postal_code: impl Into<String>) -> &mut Self {
        self.postal_code = Some(postal_code.into());
        self
    }

    /// Region name.
    pub fn region(&mut self, region: impl Into<String>) -> &mut Self {
        self.region = Some(region.into());
        self
    }

    /// Custom data to be stored with this address.
    pub fn custom_data(&mut self, custom_data: HashMap<String, String>) -> &mut Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<Address> {
        self.client
            .send(
                self,
                Method::POST,
                &format!("/customers/{}/addresses", self.customer_id.as_ref()),
            )
            .await
    }
}

/// Request builder for fetching a single address from Paddle API.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct AddressGet<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    #[serde(skip)]
    customer_id: CustomerID,
    #[serde(skip)]
    address_id: AddressID,
}

impl<'a> AddressGet<'a> {
    pub fn new(
        client: &'a Paddle,
        customer_id: impl Into<CustomerID>,
        address_id: impl Into<AddressID>,
    ) -> Self {
        Self {
            client,
            customer_id: customer_id.into(),
            address_id: address_id.into(),
        }
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<Address> {
        self.client
            .send(
                self,
                Method::GET,
                &format!(
                    "/customers/{}/addresses/{}",
                    self.customer_id.as_ref(),
                    self.address_id.as_ref()
                ),
            )
            .await
    }
}

/// Request builder for updating an address in Paddle API.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct AddressUpdate<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    #[serde(skip)]
    customer_id: CustomerID,
    #[serde(skip)]
    address_id: AddressID,
    description: Option<String>,
    first_line: Option<String>,
    second_line: Option<String>,
    city: Option<String>,
    postal_code: Option<String>,
    region: Option<String>,
    country_code: Option<CountryCodeSupported>,
    custom_data: Option<HashMap<String, String>>,
    status: Option<Status>,
}

impl<'a> AddressUpdate<'a> {
    pub fn new(
        client: &'a Paddle,
        customer_id: impl Into<CustomerID>,
        address_id: impl Into<AddressID>,
    ) -> Self {
        Self {
            client,
            customer_id: customer_id.into(),
            address_id: address_id.into(),
            description: None,
            first_line: None,
            second_line: None,
            city: None,
            postal_code: None,
            region: None,
            country_code: None,
            custom_data: None,
            status: None,
        }
    }

    /// Memorable description for this address.
    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        self.description = Some(description.into());
        self
    }

    /// First line of the address.
    pub fn first_line(&mut self, first_line: impl Into<String>) -> &mut Self {
        self.first_line = Some(first_line.into());
        self
    }

    /// Second line of the address.
    pub fn second_line(&mut self, second_line: impl Into<String>) -> &mut Self {
        self.second_line = Some(second_line.into());
        self
    }

    /// City name.
    pub fn city(&mut self, city: impl Into<String>) -> &mut Self {
        self.city = Some(city.into());
        self
    }

    /// Postal code. Required for US addresses
    pub fn postal_code(&mut self, postal_code: impl Into<String>) -> &mut Self {
        self.postal_code = Some(postal_code.into());
        self
    }

    /// Region name.
    pub fn region(&mut self, region: impl Into<String>) -> &mut Self {
        self.region = Some(region.into());
        self
    }

    /// Country code.
    pub fn country_code(&mut self, country_code: CountryCodeSupported) -> &mut Self {
        self.country_code = Some(country_code);
        self
    }

    /// Custom data to be stored with this address.
    pub fn custom_data(&mut self, custom_data: HashMap<String, String>) -> &mut Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Status of the address.
    pub fn status(&mut self, status: Status) -> &mut Self {
        self.status = Some(status);
        self
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<Address> {
        self.client
            .send(
                self,
                Method::PATCH,
                &format!(
                    "/customers/{}/addresses/{}",
                    self.customer_id.as_ref(),
                    self.address_id.as_ref()
                ),
            )
            .await
    }
}
