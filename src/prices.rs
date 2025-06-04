//! Request builders for working with price entities in Paddle API.
//!
//! See the [Paddle API](https://developer.paddle.com/api-reference/prices/overview) documentation for more information.

use std::collections::HashMap;
use std::ops::Range;

use reqwest::Method;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::entities::{Duration, Money, Price, PriceQuantity, UnitPriceOverride};
use crate::enums::{CatalogType, CountryCodeSupported, CurrencyCode, Interval, Status, TaxMode};
use crate::ids::{PriceID, ProductID};
use crate::paginated::Paginated;
use crate::{Paddle, Result};

/// Request builder for fetching prices from Paddle API.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct PricesList<'a> {
    #[serde(skip)]
    client: &'a Paddle,
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
    pub fn new(client: &'a Paddle) -> Self {
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
    pub fn after(&mut self, price_id: impl Into<PriceID>) -> &mut Self {
        self.after = Some(price_id.into());
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

    /// Returns a paginator for fetching pages of entities from Paddle
    pub fn send(&self) -> Paginated<Vec<Price>> {
        Paginated::new(self.client, "/prices", self)
    }
}

/// Request builder for creating a new price in Paddle API.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct PricesCreate<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    description: String,
    product_id: ProductID,
    unit_price: Money,
    r#type: Option<CatalogType>,
    name: Option<String>,
    billing_cycle: Option<Duration>,
    trial_period: Option<Duration>,
    tax_mode: TaxMode,
    unit_price_overrides: Option<Vec<UnitPriceOverride>>,
    quantity: Option<PriceQuantity>,
    custom_data: Option<HashMap<String, String>>,
}

impl<'a> PricesCreate<'a> {
    pub fn new(
        client: &'a Paddle,
        product_id: impl Into<ProductID>,
        description: impl Into<String>,
        amount: u64,
        currency: CurrencyCode,
    ) -> Self {
        Self {
            client,
            description: description.into(),
            product_id: product_id.into(),
            unit_price: Money {
                amount: amount.to_string(),
                currency_code: currency,
            },
            r#type: None,
            name: None,
            billing_cycle: None,
            trial_period: None,
            tax_mode: TaxMode::AccountSetting,
            unit_price_overrides: None,
            quantity: None,
            custom_data: None,
        }
    }

    /// Set the price type.
    pub fn catalog_type(&mut self, catalog_type: CatalogType) -> &mut Self {
        self.r#type = Some(catalog_type);
        self
    }

    /// Name of this price, shown to customers at checkout and on invoices. Typically describes how often the related product bills.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    /// How often this price should be charged.
    pub fn billing_cycle(&mut self, frequency: u64, interval: Interval) -> &mut Self {
        self.billing_cycle = Some(Duration {
            interval,
            frequency,
        });

        self
    }

    /// Trial period for the product related to this price. The billing cycle begins once the trial period is over. Requires billing_cycle.
    pub fn trial_period(&mut self, frequency: u64, interval: Interval) -> &mut Self {
        self.trial_period = Some(Duration {
            interval,
            frequency,
        });

        self
    }

    /// How tax is calculated for this price. If omitted, defaults to TaxMode::AccountSetting.
    /// See [TaxMode] for more information.
    pub fn tax_mode(&mut self, tax_mode: TaxMode) -> &mut Self {
        self.tax_mode = tax_mode;
        self
    }

    /// Use to override the base price with a custom price and currency for a country or group of countries.
    /// See [UnitPriceOverride] for more information.
    /// See [CountryCodeSupported] for more information.
    /// See [Money] for more information.
    /// See [CurrencyCode] for more information.
    pub fn add_unit_price_override(
        &mut self,
        country_codes: impl IntoIterator<Item = CountryCodeSupported>,
        amount: u64,
        currency: CurrencyCode,
    ) -> &mut Self {
        if self.unit_price_overrides.is_none() {
            self.unit_price_overrides = Some(vec![]);
        }

        self.unit_price_overrides
            .as_mut()
            .unwrap()
            .push(UnitPriceOverride {
                country_codes: country_codes.into_iter().collect(),
                unit_price: Money {
                    amount: amount.to_string(),
                    currency_code: currency,
                },
            });

        self
    }

    /// Use to override the base price with a custom price and currency for a country or group of countries.
    /// This will replace any existing overrides.
    /// Use `add_unit_price_override` to add additional overrides.
    /// See [UnitPriceOverride] for more information.
    /// See [CountryCodeSupported] for more information.
    /// See [Money] for more information.
    /// See [CurrencyCode] for more information.
    pub fn set_unit_price_overrides(&mut self, overrides: Vec<UnitPriceOverride>) -> &mut Self {
        self.unit_price_overrides = Some(overrides);
        self
    }

    /// Limits on how many times the related product can be purchased at this price. Useful for discount campaigns. If omitted, defaults to 1..100.
    pub fn quantity(&mut self, range: Range<u64>) -> &mut Self {
        self.quantity = Some(PriceQuantity {
            minimum: range.start,
            maximum: range.end,
        });
        self
    }

    /// Set custom data for this price.
    pub fn custom_data(&mut self, custom_data: HashMap<String, String>) -> &mut Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<Price> {
        self.client.send(self, Method::POST, "/prices").await
    }
}

/// Request builder for fetching a specific price from Paddle API.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct PriceGet<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    #[serde(skip)]
    price_id: PriceID,
    #[serde(serialize_with = "crate::comma_separated")]
    include: Option<Vec<String>>,
}

impl<'a> PriceGet<'a> {
    pub fn new(client: &'a Paddle, price_id: impl Into<PriceID>) -> Self {
        Self {
            client,
            price_id: price_id.into(),
            include: None,
        }
    }

    /// Include related entities in the response. Allowed values: "product".
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
    pub async fn send(&self) -> Result<Price> {
        self.client
            .send(
                self,
                Method::GET,
                &format!("/prices/{}", self.price_id.as_ref()),
            )
            .await
    }
}

/// Request builder for updating a price in Paddle API.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct PriceUpdate<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    #[serde(skip)]
    price_id: PriceID,
    description: Option<String>,
    r#type: Option<CatalogType>,
    name: Option<String>,
    billing_cycle: Option<Duration>,
    trial_period: Option<Duration>,
    tax_mode: Option<TaxMode>,
    unit_price: Option<Money>,
    unit_price_overrides: Option<Vec<UnitPriceOverride>>,
    quantity: Option<PriceQuantity>,
    status: Option<Status>,
    custom_data: Option<HashMap<String, String>>,
}

impl<'a> PriceUpdate<'a> {
    pub fn new(client: &'a Paddle, price_id: impl Into<PriceID>) -> Self {
        Self {
            client,
            price_id: price_id.into(),
            description: None,
            r#type: None,
            name: None,
            billing_cycle: None,
            trial_period: None,
            tax_mode: None,
            unit_price: None,
            unit_price_overrides: None,
            quantity: None,
            status: None,
            custom_data: None,
        }
    }

    /// Update the price description.
    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        self.description = Some(description.into());
        self
    }

    /// Update the price type.
    pub fn catalog_type(&mut self, catalog_type: CatalogType) -> &mut Self {
        self.r#type = Some(catalog_type);
        self
    }

    /// Update the price name. Name is shown to customers at checkout and on invoices. Typically describes how often the related product bills.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    /// Update how often this price should be charged.
    pub fn billing_cycle(&mut self, frequency: u64, interval: Interval) -> &mut Self {
        self.billing_cycle = Some(Duration {
            interval,
            frequency,
        });

        self
    }

    /// Update the trial period for the product related to this price.
    pub fn trial_period(&mut self, frequency: u64, interval: Interval) -> &mut Self {
        self.trial_period = Some(Duration {
            interval,
            frequency,
        });

        self
    }

    /// Update how tax is calculated for this price.
    pub fn tax_mode(&mut self, tax_mode: TaxMode) -> &mut Self {
        self.tax_mode = Some(tax_mode);
        self
    }

    /// Update the base price. This price applies to all customers, except for customers located in countries where you have unit_price_overrides.
    pub fn unit_price(&mut self, amount: u64, currency: CurrencyCode) -> &mut Self {
        self.unit_price = Some(Money {
            amount: amount.to_string(),
            currency_code: currency,
        });
        self
    }

    /// Use to override the base price with a custom price and currency for a country or group of countries.
    pub fn add_unit_price_override(
        &mut self,
        country_codes: impl IntoIterator<Item = CountryCodeSupported>,
        amount: u64,
        currency: CurrencyCode,
    ) -> &mut Self {
        if self.unit_price_overrides.is_none() {
            self.unit_price_overrides = Some(vec![]);
        }

        self.unit_price_overrides
            .as_mut()
            .unwrap()
            .push(UnitPriceOverride {
                country_codes: country_codes.into_iter().collect(),
                unit_price: Money {
                    amount: amount.to_string(),
                    currency_code: currency,
                },
            });

        self
    }

    /// Use to override the base price with a custom price and currency for a country or group of countries.
    pub fn set_unit_price_overrides(&mut self, overrides: Vec<UnitPriceOverride>) -> &mut Self {
        self.unit_price_overrides = Some(overrides);
        self
    }

    /// Update how many times the related product can be purchased at this price.
    pub fn quantity(&mut self, range: Range<u64>) -> &mut Self {
        self.quantity = Some(PriceQuantity {
            minimum: range.start,
            maximum: range.end,
        });
        self
    }

    /// Update whether this entity can be used in Paddle.
    pub fn status(&mut self, status: Status) -> &mut Self {
        self.status = Some(status);
        self
    }

    /// Set custom data for the price.
    pub fn custom_data(&mut self, custom_data: HashMap<String, String>) -> &mut Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<Price> {
        self.client
            .send(
                self,
                Method::PATCH,
                &format!("/prices/{}", self.price_id.as_ref()),
            )
            .await
    }
}
