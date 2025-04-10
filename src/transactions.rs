//! Builders for making requests to the Paddle API for transaction entities.
//!
//! See the [Paddle API](https://developer.paddle.com/api-reference/transactions/overview) documentation for more information.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::entities::Transaction;
use crate::enums::{CollectionMode, Status, TransactionOrigin, TransactionStatus};
use crate::ids::{CustomerID, SubscriptionID, TransactionID};
use crate::{Paddle, Result};

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Default)]
struct DateAtFilter {
    LT: Option<DateTime<Utc>>,
    LTE: Option<DateTime<Utc>>,
    GT: Option<DateTime<Utc>>,
    GTE: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum DateAt {
    Exact(DateTime<Utc>),
    Filter(DateAtFilter),
}

/// Request builder for fetching products from Paddle API.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct TransactionsList<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    after: Option<TransactionID>,
    billed_at: Option<DateAt>,
    collection_mode: Option<CollectionMode>,
    created_at: Option<DateAt>,
    #[serde(serialize_with = "crate::comma_separated")]
    customer_id: Option<Vec<CustomerID>>,
    #[serde(serialize_with = "crate::comma_separated")]
    id: Option<Vec<TransactionID>>,
    #[serde(serialize_with = "crate::comma_separated")]
    include: Option<Vec<String>>,
    #[serde(serialize_with = "crate::comma_separated")]
    invoice_number: Option<Vec<String>>,
    #[serde(serialize_with = "crate::comma_separated_enum")]
    origin: Option<Vec<TransactionOrigin>>,
    order_by: Option<String>,
    status: Option<TransactionStatus>,
    #[serde(serialize_with = "crate::comma_separated")]
    subscription_id: Option<Vec<SubscriptionID>>,
    per_page: Option<usize>,
    updated_at: Option<DateAt>,
}

impl<'a> TransactionsList<'a> {
    pub fn new(client: &'a Paddle) -> Self {
        Self {
            client,
            after: None,
            billed_at: None,
            collection_mode: None,
            created_at: None,
            customer_id: None,
            id: None,
            include: None,
            invoice_number: None,
            origin: None,
            order_by: None,
            status: None,
            subscription_id: None,
            per_page: None,
            updated_at: None,
        }
    }

    /// Return entities after the specified Paddle ID when working with paginated endpoints. Used in the `meta.pagination.next` URL in responses for list operations.
    pub fn after(&mut self, transaction_id: impl Into<TransactionID>) -> &mut Self {
        self.after = Some(transaction_id.into());
        self
    }

    /// Return entities billed at a specific time.
    pub fn billed_at(&mut self, date: DateTime<Utc>) -> &mut Self {
        self.billed_at = Some(DateAt::Exact(date));
        self
    }

    /// Return entities billed before the specified time.
    pub fn billed_at_lt(&mut self, date: DateTime<Utc>) -> &mut Self {
        self.billed_at = Some(DateAt::Filter(DateAtFilter {
            LT: Some(date),
            ..Default::default()
        }));

        self
    }

    /// Return entities billed before or on the specified time.
    pub fn billed_at_lte(&mut self, date: DateTime<Utc>) -> &mut Self {
        self.billed_at = Some(DateAt::Filter(DateAtFilter {
            LTE: Some(date),
            ..Default::default()
        }));

        self
    }

    /// Return entities billed after the specified time.
    pub fn billed_at_gt(&mut self, date: DateTime<Utc>) -> &mut Self {
        self.billed_at = Some(DateAt::Filter(DateAtFilter {
            GT: Some(date),
            ..Default::default()
        }));

        self
    }

    /// Return entities billed after or on the specified time.
    pub fn billed_at_gte(&mut self, date: DateTime<Utc>) -> &mut Self {
        self.billed_at = Some(DateAt::Filter(DateAtFilter {
            GTE: Some(date),
            ..Default::default()
        }));

        self
    }

    /// Return entities that match the specified collection mode.
    pub fn collection_mode(&mut self, mode: CollectionMode) -> &mut Self {
        self.collection_mode = Some(mode);
        self
    }

    /// Return entities created at a specific time.
    pub fn created_at(&mut self, date: DateTime<Utc>) -> &mut Self {
        self.created_at = Some(DateAt::Exact(date));
        self
    }

    /// Return entities created before the specified time.
    pub fn created_at_lt(&mut self, date: DateTime<Utc>) -> &mut Self {
        self.created_at = Some(DateAt::Filter(DateAtFilter {
            LT: Some(date),
            ..Default::default()
        }));

        self
    }

    /// Return entities created before or on the specified time.
    pub fn created_at_lte(&mut self, date: DateTime<Utc>) -> &mut Self {
        self.created_at = Some(DateAt::Filter(DateAtFilter {
            LTE: Some(date),
            ..Default::default()
        }));

        self
    }

    /// Return entities created after the specified time.
    pub fn created_at_gt(&mut self, date: DateTime<Utc>) -> &mut Self {
        self.created_at = Some(DateAt::Filter(DateAtFilter {
            GT: Some(date),
            ..Default::default()
        }));

        self
    }

    /// Return entities created after or on the specified time.
    pub fn created_at_gte(&mut self, date: DateTime<Utc>) -> &mut Self {
        self.created_at = Some(DateAt::Filter(DateAtFilter {
            GTE: Some(date),
            ..Default::default()
        }));

        self
    }

    /// Return entities related to the specified customers.
    pub fn customer_id(
        &mut self,
        customer_ids: impl IntoIterator<Item = impl Into<CustomerID>>,
    ) -> &mut Self {
        self.customer_id = Some(customer_ids.into_iter().map(Into::into).collect());
        self
    }

    /// Return only the IDs specified.
    pub fn id(&mut self, ids: impl IntoIterator<Item = impl Into<TransactionID>>) -> &mut Self {
        self.id = Some(ids.into_iter().map(Into::into).collect());
        self
    }

    /// Include related entities in the response.
    ///
    /// Valid values are:
    ///
    /// - `address`
    /// - `adjustments`
    /// - `adjustments_totals`
    /// - `available_payment_methods`
    /// - `business`
    /// - `customer`
    /// - `discount`
    ///
    pub fn include(&mut self, entities: impl IntoIterator<Item = impl AsRef<str>>) -> &mut Self {
        self.include = Some(
            entities
                .into_iter()
                .map(|s| s.as_ref().to_string())
                .collect(),
        );
        self
    }

    /// Return entities that match the invoice number.
    pub fn invoice_numbers(
        &mut self,
        numbers: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> &mut Self {
        self.invoice_number = Some(
            numbers
                .into_iter()
                .map(|s| s.as_ref().to_string())
                .collect(),
        );
        self
    }

    /// Return entities related to the specified origin(s).
    pub fn origin(&mut self, origins: impl IntoIterator<Item = TransactionOrigin>) -> &mut Self {
        self.origin = Some(origins.into_iter().collect());
        self
    }

    /// Order returned entities by the specified field. Valid fields for ordering: `billed_at`, `created_at`, `id`, `updated_at`
    pub fn order_by_asc(&mut self, field: &str) -> &mut Self {
        self.order_by = Some(format!("{}[ASC]", field));
        self
    }

    /// Order returned entities by the specified field. Valid fields for ordering: `billed_at`, `created_at`, `id`, `updated_at`
    pub fn order_by_desc(&mut self, field: &str) -> &mut Self {
        self.order_by = Some(format!("{}[DESC]", field));
        self
    }

    /// Return entities that match the specified status.
    pub fn status(&mut self, status: TransactionStatus) -> &mut Self {
        self.status = Some(status);
        self
    }

    /// Return entities related to the specified subscription.
    pub fn subscription_ids(
        &mut self,
        subscription_ids: impl IntoIterator<Item = impl Into<SubscriptionID>>,
    ) -> &mut Self {
        self.subscription_id = Some(subscription_ids.into_iter().map(Into::into).collect());
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

    /// Return entities updated at a specific time.
    pub fn updated_at(&mut self, date: DateTime<Utc>) -> &mut Self {
        self.updated_at = Some(DateAt::Exact(date));
        self
    }

    /// Return entities updated before the specified time.
    pub fn updated_at_lt(&mut self, date: DateTime<Utc>) -> &mut Self {
        self.updated_at = Some(DateAt::Filter(DateAtFilter {
            LT: Some(date),
            ..Default::default()
        }));

        self
    }

    /// Return entities updated before or on the specified time.
    pub fn updated_at_lte(&mut self, date: DateTime<Utc>) -> &mut Self {
        self.updated_at = Some(DateAt::Filter(DateAtFilter {
            LTE: Some(date),
            ..Default::default()
        }));

        self
    }

    /// Return entities updated after the specified time.
    pub fn updated_at_gt(&mut self, date: DateTime<Utc>) -> &mut Self {
        self.updated_at = Some(DateAt::Filter(DateAtFilter {
            GT: Some(date),
            ..Default::default()
        }));

        self
    }

    /// Return entities updated after or on the specified time.
    pub fn updated_at_gte(&mut self, date: DateTime<Utc>) -> &mut Self {
        self.updated_at = Some(DateAt::Filter(DateAtFilter {
            GTE: Some(date),
            ..Default::default()
        }));

        self
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<Vec<Transaction>> {
        self.client.send(self, Method::GET, "/transactions").await
    }
}
