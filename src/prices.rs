//! Builders for making requests to the Paddle API for price entities.
//!
//! See the [Paddle API](https://developer.paddle.com/api-reference/prices/overview) documentation for more information.

use std::collections::HashMap;

use reqwest::Method;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::entities::{Price, Product};
use crate::enums::{CatalogType, Status, TaxCategory};
use crate::ids::{PriceID, ProductID};
use crate::Result;

/// Request builder for fetching prices from Paddle API.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct PricesList<'a> {
    #[serde(skip)]
    client: &'a super::Paddle,
    after: Option<PriceID>,
    #[serde(serialize_with = "crate::comma_separated")]
    id: Option<Vec<PriceID>>,
    #[serde(serialize_with = "crate::comma_separated")]
    include: Option<Vec<String>>,
    order_by: Option<String>,
    per_page: Option<usize>,
    #[serde(serialize_with = "crate::comma_separated")]
    product_id: Option<Vec<ProductID>>,
    status: Option<Status>,
    recurring: Option<bool>,
    r#type: Option<CatalogType>,
}

impl<'a> PricesList<'a> {
    pub fn new(client: &'a super::Paddle) -> Self {
        Self {
            client,
            after: None,
            id: None,
            include: None,
            order_by: None,
            per_page: None,
            product_id: None,
            status: None,
            recurring: None,
            r#type: None,
        }
    }

    /// Return entities after the specified Paddle ID when working with paginated endpoints. Used in the `meta.pagination.next` URL in responses for list operations.
    pub fn after(&mut self, price_id: PriceID) -> &mut Self {
        self.after = Some(price_id);
        self
    }

    /// Return only the IDs specified.
    pub fn ids(&mut self, price_ids: impl IntoIterator<Item = impl Into<PriceID>>) -> &mut Self {
        self.id = Some(price_ids.into_iter().map(Into::into).collect());
        self
    }

    /// Include related entities in the response. Valid values are: "product".
    pub fn include(&mut self, includes: impl IntoIterator<Item = impl Into<String>>) -> &mut Self {
        self.include = Some(includes.into_iter().map(Into::into).collect());
        self
    }

    /// Order returned entities by the specified field. Valid fields for ordering: billing_cycle.frequency, billing_cycle.interval, id, product_id, quantity.maximum, quantity.minimum, status, tax_mode, unit_price.amount, and unit_price.currency_code
    pub fn order_by_asc(&mut self, field: &str) -> &mut Self {
        self.order_by = Some(format!("{}[ASC]", field));
        self
    }

    /// Order returned entities by the specified field. Valid fields for ordering: billing_cycle.frequency, billing_cycle.interval, id, product_id, quantity.maximum, quantity.minimum, status, tax_mode, unit_price.amount, and unit_price.currency_code
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

    /// Return only prices for the specified product IDs.
    pub fn product_ids(
        &mut self,
        product_ids: impl IntoIterator<Item = impl Into<ProductID>>,
    ) -> &mut Self {
        self.product_id = Some(product_ids.into_iter().map(Into::into).collect());
        self
    }

    /// Return only prices with the specified status.
    pub fn status(&mut self, status: Status) -> &mut Self {
        self.status = Some(status);
        self
    }

    /// Determine whether returned entities are for recurring prices (true) or one-time prices (false)
    pub fn recurring(&mut self, value: bool) -> &mut Self {
        self.recurring = Some(value);
        self
    }

    /// Return only prices with the specified type.
    pub fn r#type(&mut self, catalog_type: CatalogType) -> &mut Self {
        self.r#type = Some(catalog_type);
        self
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<Vec<Price>> {
        self.client.send(self, Method::GET, "/products").await
    }
}
