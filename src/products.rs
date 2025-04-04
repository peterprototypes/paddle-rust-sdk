//! Builders for making requests to the Paddle API for product entities.
//!
//! See the [Paddle API](https://developer.paddle.com/api-reference/products/overview) documentation for more information.

use std::collections::HashMap;

use reqwest::Method;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::entities::Product;
use crate::enums::{CatalogType, Status, TaxCategory};
use crate::ids::ProductID;
use crate::{Paddle, Result};

/// Request builder for fetching products from Paddle API.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct ProductsList<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    after: Option<ProductID>,
    #[serde(serialize_with = "crate::comma_separated")]
    id: Option<Vec<ProductID>>,
    #[serde(serialize_with = "crate::comma_separated")]
    include: Option<Vec<String>>,
    order_by: Option<String>,
    per_page: Option<usize>,
    status: Option<Status>,
    #[serde(serialize_with = "crate::comma_separated")]
    tax_category: Option<Vec<TaxCategory>>,
    r#type: Option<CatalogType>,
}

impl<'a> ProductsList<'a> {
    pub fn new(client: &'a Paddle) -> Self {
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

    /// Include related entities in the response. Valid values are: "prices".
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
        self.status = Some(status);
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
    pub async fn send(&self) -> Result<Vec<Product>> {
        self.client.send(self, Method::GET, "/products").await
    }
}

/// Request builder for creating a new product in Paddle API.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct ProductCreate<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    name: String,
    tax_category: TaxCategory,
    description: Option<String>,
    r#type: Option<CatalogType>,
    image_url: Option<String>,
    custom_data: Option<HashMap<String, String>>,
}

impl<'a> ProductCreate<'a> {
    pub fn new(client: &'a Paddle, name: impl Into<String>, tax_category: TaxCategory) -> Self {
        Self {
            client,
            name: name.into(),
            tax_category,
            description: None,
            r#type: None,
            image_url: None,
            custom_data: None,
        }
    }

    /// Set the product description.
    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        self.description = Some(description.into());
        self
    }

    /// Set the product catalog type.
    pub fn catalog_type(&mut self, catalog_type: CatalogType) -> &mut Self {
        self.r#type = Some(catalog_type);
        self
    }

    /// Set the product image URL.
    pub fn image_url(&mut self, image_url: impl Into<String>) -> &mut Self {
        self.image_url = Some(image_url.into());
        self
    }

    /// Set custom data for the product.
    pub fn custom_data(&mut self, custom_data: HashMap<String, String>) -> &mut Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<Product> {
        self.client.send(self, Method::POST, "/products").await
    }
}

/// Request builder for fetching a specific product from Paddle API.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct ProductGet<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    #[serde(skip)]
    product_id: ProductID,
    #[serde(serialize_with = "crate::comma_separated")]
    include: Option<Vec<String>>,
}

impl<'a> ProductGet<'a> {
    pub fn new(client: &'a Paddle, product_id: impl Into<ProductID>) -> Self {
        Self {
            client,
            product_id: product_id.into(),
            include: None,
        }
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

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<Product> {
        self.client
            .send(
                self,
                Method::GET,
                &format!("/products/{}", self.product_id.as_ref()),
            )
            .await
    }
}

/// Request builder for updating a product in Paddle API.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct ProductUpdate<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    #[serde(skip)]
    product_id: ProductID,
    name: Option<String>,
    description: Option<String>,
    r#type: Option<CatalogType>,
    tax_category: Option<TaxCategory>,
    image_url: Option<String>,
    custom_data: Option<HashMap<String, String>>,
    status: Option<Status>,
}

impl<'a> ProductUpdate<'a> {
    pub fn new(client: &'a Paddle, product_id: impl Into<ProductID>) -> Self {
        Self {
            client,
            product_id: product_id.into(),
            name: None,
            description: None,
            r#type: None,
            tax_category: None,
            image_url: None,
            custom_data: None,
            status: None,
        }
    }

    /// Set the product name.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    /// Set the product description.
    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        self.description = Some(description.into());
        self
    }

    /// Set the product catalog type.
    pub fn catalog_type(&mut self, catalog_type: CatalogType) -> &mut Self {
        self.r#type = Some(catalog_type);
        self
    }

    /// Set the product tax category.
    pub fn tax_category(&mut self, tax_category: TaxCategory) -> &mut Self {
        self.tax_category = Some(tax_category);
        self
    }

    /// Set the product image URL.
    pub fn image_url(&mut self, image_url: impl Into<String>) -> &mut Self {
        self.image_url = Some(image_url.into());
        self
    }

    /// Set custom data for the product.
    pub fn custom_data(&mut self, custom_data: HashMap<String, String>) -> &mut Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Set the product status.
    pub fn status(&mut self, status: Status) -> &mut Self {
        self.status = Some(status);
        self
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<Product> {
        self.client
            .send(
                self,
                Method::PATCH,
                &format!("/products/{}", self.product_id.as_ref()),
            )
            .await
    }
}
