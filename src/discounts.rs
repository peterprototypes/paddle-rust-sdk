//! Builders for making requests to the Paddle API for discounts.
//!
//! See the [Paddle API](https://developer.paddle.com/api-reference/discounts/overview) documentation for more information.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::entities::Discount;
use crate::enums::{CurrencyCode, DiscountType, Status};
use crate::ids::DiscountID;
use crate::{Paddle, Result};

/// Request builder for fetching discounts from Paddle API.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct DiscountsList<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    after: Option<DiscountID>,
    #[serde(serialize_with = "crate::comma_separated")]
    code: Option<Vec<String>>,
    #[serde(serialize_with = "crate::comma_separated")]
    id: Option<Vec<DiscountID>>,
    order_by: Option<String>,
    per_page: Option<usize>,
    status: Option<Status>,
}

impl<'a> DiscountsList<'a> {
    pub fn new(client: &'a Paddle) -> Self {
        Self {
            client,
            after: None,
            code: None,
            id: None,
            order_by: None,
            per_page: None,
            status: None,
        }
    }

    /// Return entities after the specified Paddle ID when working with paginated endpoints. Used in the `meta.pagination.next` URL in responses for list operations.
    pub fn after(&mut self, discount_id: DiscountID) -> &mut Self {
        self.after = Some(discount_id);
        self
    }

    /// Return only entities that match the discount codes provided
    pub fn codes(&mut self, codes: impl IntoIterator<Item = impl AsRef<str>>) -> &mut Self {
        self.code = Some(codes.into_iter().map(|s| s.as_ref().to_string()).collect());
        self
    }

    /// Return only the IDs specified.
    pub fn ids(
        &mut self,
        discount_ids: impl IntoIterator<Item = impl Into<DiscountID>>,
    ) -> &mut Self {
        self.id = Some(discount_ids.into_iter().map(Into::into).collect());
        self
    }

    /// Order returned entities by the specified field. Valid fields for ordering: created_at and id
    pub fn order_by_asc(&mut self, field: &str) -> &mut Self {
        self.order_by = Some(format!("{}[ASC]", field));
        self
    }

    /// Order returned entities by the specified field. Valid fields for ordering: created_at and id
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

    /// Return only prices with the specified status.
    pub fn status(&mut self, status: Status) -> &mut Self {
        self.status = Some(status);
        self
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<Vec<Discount>> {
        self.client.send(self, Method::GET, "/discounts").await
    }
}

/// Request builder for creating a discount in Paddle API.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct DiscountCreate<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    amount: String,
    description: String,
    r#type: DiscountType,
    enabled_for_checkout: bool,
    code: Option<String>,
    currency_code: Option<CurrencyCode>,
    recur: bool,
    maximum_recurring_intervals: Option<u64>,
    usage_limit: Option<u64>,
    restrict_to: Option<Vec<String>>,
    expires_at: Option<DateTime<Utc>>,
    custom_data: Option<HashMap<String, String>>,
}

impl<'a> DiscountCreate<'a> {
    pub fn new(
        client: &'a Paddle,
        amount: impl Into<String>,
        description: impl Into<String>,
        discount_type: DiscountType,
    ) -> Self {
        Self {
            client,
            amount: amount.into(),
            description: description.into(),
            r#type: discount_type,
            enabled_for_checkout: false,
            code: None,
            currency_code: None,
            recur: false,
            maximum_recurring_intervals: None,
            usage_limit: None,
            restrict_to: None,
            expires_at: None,
            custom_data: None,
        }
    }

    /// Whether this discount can be redeemed by customers at checkout (true) or not (false).
    pub fn enabled_for_checkout(&mut self, enabled: bool) -> &mut Self {
        self.enabled_for_checkout = enabled;
        self
    }

    /// Unique code that customers can use to redeem this discount at checkout. Use letters and numbers only, up to 32 characters. Not case-sensitive.
    ///
    /// If omitted and enabled_for_checkout is true, Paddle generates a random 10-character code.
    pub fn code(&mut self, code: impl Into<String>) -> &mut Self {
        self.code = Some(code.into());
        self
    }

    /// Supported three-letter ISO 4217 currency code. Required where discount type is [DiscountType::Flat] or [DiscountType::FlatPerSeat].
    pub fn currency_code(&mut self, currency_code: CurrencyCode) -> &mut Self {
        self.currency_code = Some(currency_code);
        self
    }

    /// Whether this discount applies for multiple subscription billing periods (`true`) or not (`false`). If omitted, defaults to `false`.
    pub fn recur(&mut self, recur: bool) -> &mut Self {
        self.recur = recur;
        self
    }

    /// Number of subscription billing periods that this discount recurs for. Requires recur. `null` if this discount recurs forever.
    pub fn maximum_recurring_intervals(&mut self, maximum_recurring_intervals: u64) -> &mut Self {
        self.maximum_recurring_intervals = Some(maximum_recurring_intervals);
        self
    }

    /// Maximum number of times this discount can be redeemed. This is an overall limit for this discount, rather than a per-customer limit. `null` if this discount can be redeemed an unlimited amount of times.
    ///
    /// Paddle counts a usage as a redemption on a checkout, transaction, or the initial application against a subscription. Transactions created for subscription renewals, midcycle changes, and one-time charges aren't considered a redemption.
    pub fn usage_limit(&mut self, usage_limit: u64) -> &mut Self {
        self.usage_limit = Some(usage_limit);
        self
    }

    /// Product or price IDs that this discount is for. When including a product ID, all prices for that product can be discounted. `null` if this discount applies to all products and prices.
    pub fn restrict_to(
        &mut self,
        restrict_to: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> &mut Self {
        self.restrict_to = Some(
            restrict_to
                .into_iter()
                .map(|s| s.as_ref().to_string())
                .collect(),
        );
        self
    }

    /// Datetime when this discount expires. Discount can no longer be redeemed after this date has elapsed. `null` if this discount can be redeemed forever.
    ///
    /// Expired discounts can't be redeemed against transactions or checkouts, but can be applied when updating subscriptions.
    pub fn expires_at(&mut self, expires_at: DateTime<Utc>) -> &mut Self {
        self.expires_at = Some(expires_at);
        self
    }

    /// Set custom data for this discount.
    pub fn custom_data(&mut self, custom_data: HashMap<String, String>) -> &mut Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<Discount> {
        self.client.send(self, Method::POST, "/discounts").await
    }
}

/// Request builder for fetching discounts from Paddle API.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct DiscountGet<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    #[serde(skip)]
    discount_id: DiscountID,
}

impl<'a> DiscountGet<'a> {
    pub fn new(client: &'a Paddle, discount_id: impl Into<DiscountID>) -> Self {
        Self {
            client,
            discount_id: discount_id.into(),
        }
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<Discount> {
        self.client
            .send(
                self,
                Method::GET,
                &format!("/discounts/{}", self.discount_id.as_ref()),
            )
            .await
    }
}
