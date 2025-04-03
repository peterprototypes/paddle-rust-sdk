//! Product entities describe the items that customers can purchase.
//!
//! Product entities are used in the [Paddle API](https://developer.paddle.com/api-reference/products/overview) to manage product information and pricing.

use reqwest::Method;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::enums::{CatalogType, Status, TaxCategory};
use crate::ids::ProductID;
use crate::Result;

/// Returns a paginated list of products. Use the query parameters to page through results.
///
/// By default, Paddle returns products that are active. Use the status query parameter to return products that are archived.
/// Use the include parameter to include related price entities in the response.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct ProductsList<'a> {
    #[serde(skip)]
    client: &'a super::Paddle,
    after: Option<ProductID>,
    #[serde(serialize_with = "crate::comma_separated")]
    id: Option<Vec<ProductID>>,
    #[serde(serialize_with = "crate::comma_separated")]
    include: Option<Vec<String>>,
    order_by: Option<String>,
    per_page: Option<usize>,
    status: Option<String>,
    #[serde(serialize_with = "crate::comma_separated")]
    tax_category: Option<Vec<TaxCategory>>,
    r#type: Option<CatalogType>,
}

impl<'a> ProductsList<'a> {
    pub fn new(client: &'a super::Paddle) -> Self {
        Self {
            client,
            after: None,
            id: None,
            include: None,
            order_by: None,
            per_page: None,
            status: None,
            tax_category: None,
            r#type: None,
        }
    }

    /// Return entities after the specified Paddle ID when working with paginated endpoints. Used in the `meta.pagination.next` URL in responses for list operations.
    pub fn after(&mut self, product_id: ProductID) -> &mut Self {
        self.after = Some(product_id);
        self
    }

    /// Return only the IDs specified.
    pub fn ids(
        &mut self,
        product_ids: impl IntoIterator<Item = impl Into<ProductID>>,
    ) -> &mut Self {
        self.id = Some(product_ids.into_iter().map(|i| i.into()).collect());
        self
    }

    /// Include related entities in the response.
    pub fn include(&mut self, entities: impl IntoIterator<Item = impl AsRef<str>>) -> &mut Self {
        self.include = Some(
            entities
                .into_iter()
                .map(|s| s.as_ref().to_string())
                .collect(),
        );
        self
    }

    /// Order returned entities by the specified field. Valid fields for ordering: `created_at`, `custom_data`, `description`, `id`, `image_url`, `name`, `status`, `tax_category`, and `updated_at`.
    pub fn order_by_asc(&mut self, field: &str) -> &mut Self {
        self.order_by = Some(format!("{}[ASC]", field));
        self
    }

    /// Order returned entities by the specified field. Valid fields for ordering: `created_at`, `custom_data`, `description`, `id`, `image_url`, `name`, `status`, `tax_category`, and `updated_at`.
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

    /// Return entities that match the specified status.
    pub fn status(&mut self, status: Status) -> &mut Self {
        self.status = Some(match status {
            Status::Active => "active".to_string(),
            Status::Archived => "archived".to_string(),
        });

        self
    }

    /// Return entities that match the specified tax categories.
    pub fn tax_category(
        &mut self,
        tax_categories: impl IntoIterator<Item = TaxCategory>,
    ) -> &mut Self {
        self.tax_category = Some(tax_categories.into_iter().collect());
        self
    }

    /// Return entities that match the specified catalog type.
    pub fn catalog_type(&mut self, catalog_type: CatalogType) -> &mut Self {
        self.r#type = Some(catalog_type);
        self
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<Vec<crate::entities::Product>> {
        self.client.send(self, Method::GET, "/products").await
    }
}
