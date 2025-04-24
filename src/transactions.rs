//! Builders for making requests to the Paddle API for transaction entities.
//!
//! See the [Paddle API](https://developer.paddle.com/api-reference/transactions/overview) documentation for more information.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::entities::{
    BillingDetails, TimePeriod, Transaction, TransactionCheckout, TransactionItemNonCatalogPrice,
};
use crate::enums::{CollectionMode, CurrencyCode, TransactionOrigin, TransactionStatus};
use crate::ids::{
    AddressID, BusinessID, CustomerID, DiscountID, PriceID, SubscriptionID, TransactionID,
};
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

/// Request builder for fetching transactions from Paddle API.
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

#[derive(Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum TransactionItem {
    CatalogItem {
        price_id: PriceID,
        quantity: u32,
    },
    NonCatalogItem {
        price: TransactionItemNonCatalogPrice,
        quantity: u32,
    },
}

/// Request builder for creating a transaction in Paddle.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct TransactionCreate<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    #[serde(skip)]
    include: Option<Vec<String>>,
    items: Vec<TransactionItem>,
    status: Option<TransactionStatus>,
    customer_id: Option<CustomerID>,
    address_id: Option<AddressID>,
    business_id: Option<BusinessID>,
    custom_data: Option<HashMap<String, String>>,
    currency_code: Option<CurrencyCode>,
    collection_mode: Option<CollectionMode>,
    discount_id: Option<DiscountID>,
    billing_details: Option<BillingDetails>,
    billing_period: Option<TimePeriod>,
    checkout: Option<TransactionCheckout>,
}

impl<'a> TransactionCreate<'a> {
    pub fn new(client: &'a Paddle) -> Self {
        Self {
            client,
            include: None,
            items: Vec::default(),
            status: None,
            customer_id: None,
            address_id: None,
            business_id: None,
            custom_data: None,
            currency_code: None,
            collection_mode: None,
            discount_id: None,
            billing_details: None,
            billing_period: None,
            checkout: None,
        }
    }

    /// Include related entities in the response.
    ///
    /// ## Valid values are:
    ///
    /// - `address`
    /// - `adjustments`
    /// - `adjustments_totals`
    /// - `available_payment_methods`
    /// - `business`
    /// - `customer`
    /// - `discount`
    pub fn include(&mut self, includes: impl IntoIterator<Item = impl Into<String>>) -> &mut Self {
        self.include = Some(includes.into_iter().map(Into::into).collect());
        self
    }

    /// Append to the list of items to charge for.
    ///
    /// You can charge for items that you've added to your catalog by passing the Paddle ID of an existing price entity,
    ///
    /// To charge for non-catalog items see append_non_catalog_item.
    pub fn append_catalog_item(
        &mut self,
        price_id: impl Into<PriceID>,
        quantity: u32,
    ) -> &mut Self {
        self.items.push(TransactionItem::CatalogItem {
            price_id: price_id.into(),
            quantity,
        });

        self
    }

    /// Append to the list of items to charge for.
    ///
    /// You can charge for non-catalog items by passing a `TransactionItemNonCatalogPrice` object.
    pub fn append_non_catalog_item(
        &mut self,
        price: TransactionItemNonCatalogPrice,
        quantity: u32,
    ) -> &mut Self {
        self.items
            .push(TransactionItem::NonCatalogItem { price, quantity });
        self
    }

    /// Status of this transaction. You may set a transaction to billed when creating, or omit to let Paddle set the status.
    ///
    /// Transactions are created as ready if they have an address_id, customer_id, and items, otherwise they are created as draft.
    ///
    /// Marking as billed when creating is typically used when working with manually-collected transactions as part of an invoicing workflow. Billed transactions cannot be updated, only canceled.
    pub fn status(&mut self, status: TransactionStatus) -> &mut Self {
        self.status = Some(status);
        self
    }

    /// Paddle ID of the customer that this transaction is for.
    ///
    /// If omitted, transaction status is `draft`.
    pub fn customer_id(&mut self, customer_id: impl Into<CustomerID>) -> &mut Self {
        self.customer_id = Some(customer_id.into());
        self
    }

    /// Paddle ID of the address that this transaction is for.
    ///
    /// Requires customer_id. If omitted, transaction status is draft.
    pub fn address_id(&mut self, address_id: impl Into<AddressID>) -> &mut Self {
        self.address_id = Some(address_id.into());
        self
    }

    /// Paddle ID of the business that this transaction is for.
    ///
    /// Requires customer_id
    pub fn business_id(&mut self, business_id: impl Into<BusinessID>) -> &mut Self {
        self.business_id = Some(business_id.into());
        self
    }

    /// Your own structured key-value data.
    pub fn custom_data(&mut self, custom_data: HashMap<String, String>) -> &mut Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Supported three-letter ISO 4217 currency code. Must be `USD`, `EUR`, or `GBP` if `collection_mode` is `manual`.
    pub fn currency_code(&mut self, currency_code: CurrencyCode) -> &mut Self {
        self.currency_code = Some(currency_code);
        self
    }

    /// How payment is collected for this transaction. `automatic` for checkout, `manual` for invoices. If omitted, defaults to `automatic`.
    pub fn collection_mode(&mut self, mode: CollectionMode) -> &mut Self {
        self.collection_mode = Some(mode);
        self
    }

    /// Paddle ID of the discount applied to this transaction.
    pub fn discount_id(&mut self, discount_id: impl Into<DiscountID>) -> &mut Self {
        self.discount_id = Some(discount_id.into());
        self
    }

    /// Details for invoicing. Required if `collection_mode` is `manual`.
    pub fn billing_details(&mut self, billing_details: BillingDetails) -> &mut Self {
        self.billing_details = Some(billing_details);
        self
    }

    /// Time period that this transaction is for. Set automatically by Paddle for subscription renewals to describe the period that charges are for.
    pub fn billing_period(&mut self, billing_period: TimePeriod) -> &mut Self {
        self.billing_period = Some(billing_period);
        self
    }

    /// Paddle Checkout URL for creating or updating an automatically-collected transaction, or when creating or updating a manually-collected transaction
    /// where `billing_details.enable_checkout` is `true`.
    ///
    /// Pass the URL for an approved domain, or null to set to your default payment URL.
    ///
    /// Paddle returns a unique payment link composed of the URL passed or your default payment URL + ?_ptxn= and the Paddle ID for this transaction.
    pub fn checkout_url(&mut self, url: String) -> &mut Self {
        self.checkout = Some(TransactionCheckout { url: Some(url) });
        self
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<Transaction> {
        let url = if let Some(include) = self.include.as_ref() {
            &format!("/transactions?include={}", include.join(","))
        } else {
            "/transactions"
        };

        self.client.send(self, Method::POST, url).await
    }
}

/// Request builder for fetching a specific transaction.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct TransactionGet<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    #[serde(skip)]
    transaction_id: TransactionID,
    #[serde(serialize_with = "crate::comma_separated")]
    include: Option<Vec<String>>,
}

impl<'a> TransactionGet<'a> {
    pub fn new(client: &'a Paddle, transaction_id: impl Into<TransactionID>) -> Self {
        Self {
            client,
            transaction_id: transaction_id.into(),
            include: None,
        }
    }

    /// Include related entities in the response.
    ///
    /// ## Valid values are:
    ///
    /// - `address`
    /// - `adjustments`
    /// - `adjustments_totals`
    /// - `available_payment_methods`
    /// - `business`
    /// - `customer`
    /// - `discount`
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
    pub async fn send(&self) -> Result<Transaction> {
        self.client
            .send(
                self,
                Method::GET,
                &format!("/transactions/{}", self.transaction_id.as_ref()),
            )
            .await
    }
}

/// Request builder for updating a transaction.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct TransactionUpdate<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    #[serde(skip)]
    transaction_id: TransactionID,
    #[serde(skip)]
    include: Option<Vec<String>>,
    status: Option<TransactionStatus>,
    customer_id: Option<CustomerID>,
    address_id: Option<AddressID>,
    business_id: Option<BusinessID>,
    custom_data: Option<HashMap<String, String>>,
    currency_code: Option<CurrencyCode>,
    collection_mode: Option<CollectionMode>,
    discount_id: Option<DiscountID>,
    billing_details: Option<BillingDetails>,
    billing_period: Option<TimePeriod>,
    items: Option<Vec<TransactionItem>>,
    checkout: Option<TransactionCheckout>,
}

impl<'a> TransactionUpdate<'a> {
    pub fn new(client: &'a Paddle, transaction_id: impl Into<TransactionID>) -> Self {
        Self {
            client,
            transaction_id: transaction_id.into(),
            include: None,
            status: None,
            customer_id: None,
            address_id: None,
            business_id: None,
            custom_data: None,
            currency_code: None,
            collection_mode: None,
            discount_id: None,
            billing_details: None,
            billing_period: None,
            items: None,
            checkout: None,
        }
    }

    /// Include related entities in the response.
    ///
    /// ## Valid values are:
    ///
    /// - `address`
    /// - `adjustments`
    /// - `adjustments_totals`
    /// - `available_payment_methods`
    /// - `business`
    /// - `customer`
    /// - `discount`
    pub fn include(&mut self, entities: impl IntoIterator<Item = impl AsRef<str>>) -> &mut Self {
        self.include = Some(
            entities
                .into_iter()
                .map(|s| s.as_ref().to_string())
                .collect(),
        );
        self
    }

    /// Status of this transaction. You may set a transaction to billed or canceled. Billed transactions cannot be changed.
    ///
    /// For manually-collected transactions, marking as billed is essentially issuing an invoice.
    pub fn status(&mut self, status: TransactionStatus) -> &mut Self {
        self.status = Some(status);
        self
    }

    /// Paddle ID of the customer that this transaction is for.
    pub fn customer_id(&mut self, customer_id: impl Into<CustomerID>) -> &mut Self {
        self.customer_id = Some(customer_id.into());
        self
    }

    /// Paddle ID of the address that this transaction is for.
    pub fn address_id(&mut self, address_id: impl Into<AddressID>) -> &mut Self {
        self.address_id = Some(address_id.into());
        self
    }

    /// Paddle ID of the business that this transaction is for.
    pub fn business_id(&mut self, business_id: impl Into<BusinessID>) -> &mut Self {
        self.business_id = Some(business_id.into());
        self
    }

    /// Your own structured key-value data.
    pub fn custom_data(&mut self, custom_data: HashMap<String, String>) -> &mut Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Supported three-letter currency code. Must be `USD`, `EUR`, or `GBP` if `collection_mode` is `manual`.
    pub fn currency_code(&mut self, currency_code: CurrencyCode) -> &mut Self {
        self.currency_code = Some(currency_code);
        self
    }

    /// How payment is collected for this transaction. `automatic` for checkout, `manual` for invoices.
    pub fn collection_mode(&mut self, mode: CollectionMode) -> &mut Self {
        self.collection_mode = Some(mode);
        self
    }

    /// Paddle ID of the discount applied to this transaction.
    pub fn discount_id(&mut self, discount_id: impl Into<DiscountID>) -> &mut Self {
        self.discount_id = Some(discount_id.into());
        self
    }

    /// Details for invoicing. Required if `collection_mode` is `manual`.
    pub fn billing_details(&mut self, billing_details: BillingDetails) -> &mut Self {
        self.billing_details = Some(billing_details);
        self
    }

    /// Time period that this transaction is for. Set automatically by Paddle for subscription renewals to describe the period that charges are for.
    pub fn billing_period(&mut self, billing_period: TimePeriod) -> &mut Self {
        self.billing_period = Some(billing_period);
        self
    }

    pub fn items(&mut self, items: impl IntoIterator<Item = TransactionItem>) -> &mut Self {
        self.items = Some(items.into_iter().collect());
        self
    }

    /// Paddle Checkout URL for creating or updating an automatically-collected transaction, or when creating or updating a manually-collected transaction
    /// where `billing_details.enable_checkout` is `true`.
    ///
    /// Pass the URL for an approved domain, or null to set to your default payment URL.
    ///
    /// Paddle returns a unique payment link composed of the URL passed or your default payment URL + ?_ptxn= and the Paddle ID for this transaction.
    pub fn checkout_url(&mut self, url: String) -> &mut Self {
        self.checkout = Some(TransactionCheckout { url: Some(url) });
        self
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<Transaction> {
        let mut url = format!("/transactions/{}", self.transaction_id.as_ref());

        if let Some(include) = self.include.as_ref() {
            url.push_str(&format!("?include={}", include.join(",")));
        }

        self.client.send(self, Method::PATCH, &url).await
    }
}
