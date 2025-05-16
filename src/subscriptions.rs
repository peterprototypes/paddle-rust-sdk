//! Builders for making requests to the Paddle API for subscription entities.
//!
//! See the [Paddle API](https://developer.paddle.com/api-reference/subscriptions/overview) documentation for more information.

use chrono::prelude::*;
use reqwest::Method;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::entities::{
    BillingDetails, Subscription, SubscriptionDiscountEffectiveFrom, SubscriptionPreview,
    Transaction,
};
use crate::enums::{
    CollectionMode, CurrencyCode, ProrationBillingMode, ScheduledChangeAction,
    SubscriptionOnPaymentFailure, SubscriptionStatus,
};
use crate::ids::{AddressID, BusinessID, CustomerID, PriceID, SubscriptionID};
use crate::transactions::TransactionItem;
use crate::{Paddle, Result};

/// Request builder for fetching subscriptions from Paddle API.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct SubscriptionsList<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    #[serde(serialize_with = "crate::comma_separated")]
    address_id: Option<Vec<AddressID>>,
    after: Option<SubscriptionID>,
    collection_mode: Option<CollectionMode>,
    #[serde(serialize_with = "crate::comma_separated")]
    customer_id: Option<Vec<CustomerID>>,
    #[serde(serialize_with = "crate::comma_separated")]
    id: Option<Vec<SubscriptionID>>,
    order_by: Option<String>,
    per_page: Option<usize>,
    #[serde(serialize_with = "crate::comma_separated")]
    price_id: Option<Vec<PriceID>>,
    #[serde(serialize_with = "crate::comma_separated_enum")]
    scheduled_change_action: Option<Vec<ScheduledChangeAction>>,
    #[serde(serialize_with = "crate::comma_separated_enum")]
    status: Option<Vec<SubscriptionStatus>>,
}

impl<'a> SubscriptionsList<'a> {
    pub fn new(client: &'a Paddle) -> Self {
        Self {
            client,
            address_id: None,
            after: None,
            collection_mode: None,
            customer_id: None,
            id: None,
            order_by: None,
            per_page: None,
            price_id: None,
            scheduled_change_action: None,
            status: None,
        }
    }

    /// Return entities related to the specified addresses.
    pub fn address_ids(
        &mut self,
        address_ids: impl IntoIterator<Item = impl Into<AddressID>>,
    ) -> &mut Self {
        self.address_id = Some(address_ids.into_iter().map(Into::into).collect());
        self
    }

    /// Return entities after the specified Paddle ID when working with paginated endpoints. Used in the `meta.pagination.next` URL in responses for list operations.
    pub fn after(&mut self, id: impl Into<SubscriptionID>) -> &mut Self {
        self.after = Some(id.into());
        self
    }

    /// Return entities that match the specified collection mode.
    pub fn collection_mode(&mut self, mode: CollectionMode) -> &mut Self {
        self.collection_mode = Some(mode);
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
    pub fn id(&mut self, ids: impl IntoIterator<Item = impl Into<SubscriptionID>>) -> &mut Self {
        self.id = Some(ids.into_iter().map(Into::into).collect());
        self
    }

    /// Order returned entities by the specified field. Valid fields for ordering: `id`
    pub fn order_by_asc(&mut self, field: &str) -> &mut Self {
        self.order_by = Some(format!("{}[ASC]", field));
        self
    }

    /// Order returned entities by the specified field. Valid fields for ordering: `id`
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

    /// Return entities related to the specified prices.
    pub fn price_ids(
        &mut self,
        price_ids: impl IntoIterator<Item = impl Into<PriceID>>,
    ) -> &mut Self {
        self.price_id = Some(price_ids.into_iter().map(Into::into).collect());
        self
    }

    /// Return subscriptions that have a scheduled changes.
    pub fn scheduled_change_action(
        &mut self,
        actions: impl IntoIterator<Item = ScheduledChangeAction>,
    ) -> &mut Self {
        self.scheduled_change_action = Some(actions.into_iter().collect());
        self
    }

    /// Return entities that match the specified subscription statuses.
    pub fn status(&mut self, statuses: impl IntoIterator<Item = SubscriptionStatus>) -> &mut Self {
        self.status = Some(statuses.into_iter().collect());
        self
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<Vec<Subscription>> {
        self.client.send(self, Method::GET, "/subscriptions").await
    }
}

/// Request builder for fetching a specific subscription.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct SubscriptionGet<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    #[serde(skip)]
    subscription_id: SubscriptionID,
    #[serde(serialize_with = "crate::comma_separated")]
    include: Option<Vec<String>>,
}

impl<'a> SubscriptionGet<'a> {
    pub fn new(client: &'a Paddle, subscription_id: impl Into<SubscriptionID>) -> Self {
        Self {
            client,
            subscription_id: subscription_id.into(),
            include: None,
        }
    }

    /// Include related entities in the response.
    ///
    /// ## Valid values are:
    ///
    /// - `next_transaction` - Include an object with a preview of the next transaction for this subscription. May include prorated charges that aren't yet billed and one-time charges.
    /// - `recurring_transaction_details` - Include an object with a preview of the recurring transaction for this subscription. This is what the customer can expect to be billed when there are no prorated or one-time charges.
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

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<Subscription> {
        self.client
            .send(
                self,
                Method::GET,
                &format!("/subscriptions/{}", self.subscription_id.as_ref()),
            )
            .await
    }
}

// Note: Unlike other structs we cannot use this directly for the preview request because we need to
// serialize null values to indicate that they should be removed from the subscription preview.

/// Request builder for getting a preview of changes to a subscription without actually applying them.
///
/// Typically used for previewing proration before making changes to a subscription.
pub struct SubscriptionPreviewUpdate<'a> {
    client: &'a Paddle,
    subscription_id: SubscriptionID,
    data: serde_json::Value,
}

impl<'a> SubscriptionPreviewUpdate<'a> {
    pub fn new(client: &'a Paddle, subscription_id: impl Into<SubscriptionID>) -> Self {
        Self {
            client,
            subscription_id: subscription_id.into(),
            data: serde_json::json!({}),
        }
    }

    /// The customer ID to use for the preview. Include to change the customer for a subscription.
    pub fn customer_id(&mut self, customer_id: impl Into<CustomerID>) -> &mut Self {
        self.data["customer_id"] = serde_json::json!(customer_id.into());
        self
    }

    /// The address ID to use for the preview. Include to change the address for a subscription.
    pub fn address_id(&mut self, address_id: impl Into<AddressID>) -> &mut Self {
        self.data["address_id"] = serde_json::json!(address_id.into());
        self
    }

    /// The business ID to use for the preview. Include to change the business for a subscription.
    pub fn business_id(&mut self, business_id: impl Into<BusinessID>) -> &mut Self {
        self.data["business_id"] = serde_json::json!(business_id.into());
        self
    }

    /// Supported currency code. Include to change the currency that a subscription bills in.
    ///
    /// When changing `collection_mode` to `manual`, you may need to change currency code to `USD`, `EUR`, or `GBP`.
    pub fn currency_code(&mut self, currency_code: CurrencyCode) -> &mut Self {
        self.data["currency_code"] = serde_json::json!(currency_code);
        self
    }

    /// Datetime of when this subscription is next scheduled to be billed. Include to change the next billing date.
    pub fn next_billed_at(&mut self, next_billed_at: DateTime<Utc>) -> &mut Self {
        self.data["next_billed_at"] = serde_json::json!(next_billed_at);
        self
    }

    /// Details of the discount applied to this subscription. Include to add a discount to a subscription. None to remove a discount.
    pub fn set_discount(
        &mut self,
        discount: Option<SubscriptionDiscountEffectiveFrom>,
    ) -> &mut Self {
        self.data["discount"] = serde_json::json!(discount);
        self
    }

    /// How payment is collected for transactions created for this subscription. `automatic` for checkout, `manual` for invoices.
    pub fn collection_mode(&mut self, mode: CollectionMode) -> &mut Self {
        self.data["collection_mode"] = serde_json::json!(mode);
        self
    }

    /// Details for invoicing. Required if `collection_mode` is `manual`. `None` if changing `collection_mode` to `automatic`.
    pub fn billing_details(&mut self, billing_details: Option<BillingDetails>) -> &mut Self {
        self.data["billing_details"] = serde_json::json!(billing_details);
        self
    }

    /// Change that's scheduled to be applied to a subscription.
    ///
    /// When updating, you may only set to `null` to remove a scheduled change.
    ///
    /// Use the pause subscription, cancel subscription, and resume subscription operations to create scheduled changes.
    pub fn unset_scheduled_change(&mut self) -> &mut Self {
        self.data["scheduled_change"] = serde_json::json!(null);
        self
    }

    /// List of items on this subscription. Only recurring items may be added. Send the complete list of items that should be on this subscription, including existing items to retain.
    pub fn items(&mut self, items: impl IntoIterator<Item = TransactionItem>) -> &mut Self {
        self.data["items"] = serde_json::json!(items.into_iter().collect::<Vec<_>>());
        self
    }

    /// Your own structured key-value data.
    pub fn custom_data(&mut self, custom_data: serde_json::Value) -> &mut Self {
        self.data["custom_data"] = custom_data;
        self
    }

    /// How Paddle should handle proration calculation for changes made to a subscription or its items. Required when making changes that impact billing.
    ///
    /// For automatically-collected subscriptions, responses may take longer than usual if a proration billing mode that collects for payment immediately is used.
    pub fn proration_billing_mode(&mut self, mode: ProrationBillingMode) -> &mut Self {
        self.data["proration_billing_mode"] = serde_json::json!(mode);
        self
    }

    /// How Paddle should handle changes made to a subscription or its items if the payment fails during update. If omitted, defaults to `prevent_change`.
    pub fn on_payment_failure(&mut self, mode: SubscriptionOnPaymentFailure) -> &mut Self {
        self.data["on_payment_failure"] = serde_json::json!(mode);
        self
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<SubscriptionPreview> {
        self.client
            .send(
                &self.data,
                Method::PATCH,
                &format!("/subscriptions/{}/preview", self.subscription_id.as_ref()),
            )
            .await
    }
}

// Note: Unlike other structs we cannot use this directly for the preview request because we need to
// serialize null values to indicate that they should be removed from the subscription preview.

/// Request builder for updating a subscription using its ID.
///
/// When making changes to items or the next billing date for a subscription, you must include the `proration_billing_mode` field to tell Paddle how to bill for those changes.
///
/// Send the complete list of items that you'd like to be on a subscription — including existing items. If you omit items, they're removed from the subscription.
///
/// For each item, send `price_id` and `quantity`. Paddle responds with the full price object for each price. If you're updating an existing item, you can omit the `quantity` if you don't want to update it.
///
/// If successful, your response includes a copy of the updated subscription entity. When an update results in an immediate charge, responses may take longer than usual while a payment attempt is processed.
pub struct SubscriptionUpdate<'a> {
    client: &'a Paddle,
    subscription_id: SubscriptionID,
    data: serde_json::Value,
}

impl<'a> SubscriptionUpdate<'a> {
    pub fn new(client: &'a Paddle, subscription_id: impl Into<SubscriptionID>) -> Self {
        Self {
            client,
            subscription_id: subscription_id.into(),
            data: serde_json::json!({}),
        }
    }

    /// The customer ID to use for the preview. Include to change the customer for a subscription.
    pub fn customer_id(&mut self, customer_id: impl Into<CustomerID>) -> &mut Self {
        self.data["customer_id"] = serde_json::json!(customer_id.into());
        self
    }

    /// The address ID to use for the preview. Include to change the address for a subscription.
    pub fn address_id(&mut self, address_id: impl Into<AddressID>) -> &mut Self {
        self.data["address_id"] = serde_json::json!(address_id.into());
        self
    }

    /// The business ID to use for the preview. Include to change the business for a subscription.
    pub fn business_id(&mut self, business_id: impl Into<BusinessID>) -> &mut Self {
        self.data["business_id"] = serde_json::json!(business_id.into());
        self
    }

    /// Supported currency code. Include to change the currency that a subscription bills in.
    ///
    /// When changing `collection_mode` to `manual`, you may need to change currency code to `USD`, `EUR`, or `GBP`.
    pub fn currency_code(&mut self, currency_code: CurrencyCode) -> &mut Self {
        self.data["currency_code"] = serde_json::json!(currency_code);
        self
    }

    /// Datetime of when this subscription is next scheduled to be billed. Include to change the next billing date.
    pub fn next_billed_at(&mut self, next_billed_at: DateTime<Utc>) -> &mut Self {
        self.data["next_billed_at"] = serde_json::json!(next_billed_at);
        self
    }

    /// Details of the discount applied to this subscription. Include to add a discount to a subscription. None to remove a discount.
    pub fn set_discount(
        &mut self,
        discount: Option<SubscriptionDiscountEffectiveFrom>,
    ) -> &mut Self {
        self.data["discount"] = serde_json::json!(discount);
        self
    }

    /// How payment is collected for transactions created for this subscription. `automatic` for checkout, `manual` for invoices.
    pub fn collection_mode(&mut self, mode: CollectionMode) -> &mut Self {
        self.data["collection_mode"] = serde_json::json!(mode);
        self
    }

    /// Details for invoicing. Required if `collection_mode` is `manual`. `None` if changing `collection_mode` to `automatic`.
    pub fn billing_details(&mut self, billing_details: Option<BillingDetails>) -> &mut Self {
        self.data["billing_details"] = serde_json::json!(billing_details);
        self
    }

    /// Change that's scheduled to be applied to a subscription.
    ///
    /// When updating, you may only set to `null` to remove a scheduled change.
    ///
    /// Use the pause subscription, cancel subscription, and resume subscription operations to create scheduled changes.
    pub fn unset_scheduled_change(&mut self) -> &mut Self {
        self.data["scheduled_change"] = serde_json::json!(null);
        self
    }

    /// List of items on this subscription. Only recurring items may be added. Send the complete list of items that should be on this subscription, including existing items to retain.
    pub fn items(&mut self, items: impl IntoIterator<Item = TransactionItem>) -> &mut Self {
        self.data["items"] = serde_json::json!(items.into_iter().collect::<Vec<_>>());
        self
    }

    /// Your own structured key-value data.
    pub fn custom_data(&mut self, custom_data: serde_json::Value) -> &mut Self {
        self.data["custom_data"] = custom_data;
        self
    }

    /// How Paddle should handle proration calculation for changes made to a subscription or its items. Required when making changes that impact billing.
    ///
    /// For automatically-collected subscriptions, responses may take longer than usual if a proration billing mode that collects for payment immediately is used.
    pub fn proration_billing_mode(&mut self, mode: ProrationBillingMode) -> &mut Self {
        self.data["proration_billing_mode"] = serde_json::json!(mode);
        self
    }

    /// How Paddle should handle changes made to a subscription or its items if the payment fails during update. If omitted, defaults to `prevent_change`.
    pub fn on_payment_failure(&mut self, mode: SubscriptionOnPaymentFailure) -> &mut Self {
        self.data["on_payment_failure"] = serde_json::json!(mode);
        self
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<Subscription> {
        self.client
            .send(
                &self.data,
                Method::PATCH,
                &format!("/subscriptions/{}", self.subscription_id.as_ref()),
            )
            .await
    }
}
