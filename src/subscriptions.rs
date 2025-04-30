//! Builders for making requests to the Paddle API for subscription entities.
//!
//! See the [Paddle API](https://developer.paddle.com/api-reference/subscriptions/overview) documentation for more information.

use reqwest::Method;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::entities::Subscription;
use crate::enums::{CollectionMode, ScheduledChangeAction, SubscriptionStatus};
use crate::ids::{AddressID, CustomerID, PriceID, SubscriptionID};
use crate::{Paddle, Result};

/// Request builder for fetching transactions from Paddle API.
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

/// Request builder for fetching a specific transaction.
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
