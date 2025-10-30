//! Builders for making requests to the Paddle API for making adjustments to billed or completed transactions.
//!
//! See the [Paddle API](https://developer.paddle.com/api-reference/adjustments/overview) documentation for more information.

use reqwest::Method;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::entities::{Adjustment, AdjustmentItemInput};
use crate::enums::{AdjustmentAction, AdjustmentStatus, AdjustmentType, TaxMode};
use crate::ids::{AdjustmentID, CustomerID, SubscriptionID, TransactionID};
use crate::paginated::Paginated;
use crate::{Paddle, Result};

// Request builder for retrieving adjustments
#[skip_serializing_none]
#[derive(Serialize)]
pub struct AdjustmentsList<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    action: Option<AdjustmentAction>,
    after: Option<AdjustmentID>,
    #[serde(serialize_with = "crate::comma_separated")]
    customer_id: Option<Vec<CustomerID>>,
    order_by: Option<String>,
    per_page: Option<usize>,
    #[serde(serialize_with = "crate::comma_separated_enum")]
    status: Option<Vec<AdjustmentStatus>>,
    #[serde(serialize_with = "crate::comma_separated")]
    subscription_id: Option<Vec<SubscriptionID>>,
    #[serde(serialize_with = "crate::comma_separated")]
    transaction_id: Option<Vec<TransactionID>>,
    #[serde(serialize_with = "crate::comma_separated")]
    id: Option<Vec<AdjustmentID>>,
}

impl<'a> AdjustmentsList<'a> {
    pub fn new(client: &'a Paddle) -> Self {
        Self {
            client,
            action: None,
            after: None,
            customer_id: None,
            order_by: None,
            per_page: None,
            status: None,
            subscription_id: None,
            transaction_id: None,
            id: None,
        }
    }

    /// Return entities for the specified action.
    pub fn action(&mut self, adjustment_action: AdjustmentAction) -> &mut Self {
        self.action = Some(adjustment_action);
        self
    }

    /// Return entities after the specified Paddle ID when working with paginated endpoints. Used in the `meta.pagination.next` URL in responses for list operations.
    pub fn after(&mut self, id: impl Into<AdjustmentID>) -> &mut Self {
        self.after = Some(id.into());
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

    /// Return entities that match the specified status.
    pub fn status(&mut self, statuses: impl IntoIterator<Item = AdjustmentStatus>) -> &mut Self {
        self.status = Some(statuses.into_iter().collect());
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

    /// Return entities related to the specified subscription.
    pub fn transaction_ids(
        &mut self,
        transaction_ids: impl IntoIterator<Item = impl Into<TransactionID>>,
    ) -> &mut Self {
        self.transaction_id = Some(transaction_ids.into_iter().map(Into::into).collect());
        self
    }

    /// Return only the IDs specified.
    pub fn id(&mut self, ids: impl IntoIterator<Item = impl Into<AdjustmentID>>) -> &mut Self {
        self.id = Some(ids.into_iter().map(Into::into).collect());
        self
    }

    /// Returns a paginator for fetching pages of entities from Paddle
    pub fn send(&self) -> Paginated<'_, Vec<Adjustment>> {
        Paginated::new(self.client, "/adjustments", self)
    }
}

/// Request builder for creating an adjustment in Paddle.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct AdjustmentCreate<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    transaction_id: TransactionID,
    action: AdjustmentAction,
    reason: String,
    r#type: Option<AdjustmentType>,
    tax_mode: Option<TaxMode>,
    items: Option<Vec<AdjustmentItemInput>>,
}

impl<'a> AdjustmentCreate<'a> {
    pub fn new(
        client: &'a Paddle,
        transaction_id: impl Into<TransactionID>,
        action: AdjustmentAction,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            client,
            transaction_id: transaction_id.into(),
            action,
            reason: reason.into(),
            r#type: None,
            tax_mode: None,
            items: None,
        }
    }

    /// Type of adjustment. Use `full` to adjust the grand total for the related transaction. Include an `items` array when creating a `partial` adjustment. If omitted, defaults to `partial`.
    pub fn r#type(&mut self, adjustment_type: AdjustmentType) -> &mut Self {
        self.r#type = Some(adjustment_type);
        self
    }

    /// List of transaction items to adjust. Required if `type` is not populated or set to `partial`.
    pub fn items(&mut self, items: impl IntoIterator<Item = AdjustmentItemInput>) -> &mut Self {
        self.items = Some(items.into_iter().collect());
        self
    }

    /// Whether the amounts to be adjusted are inclusive or exclusive of tax. If `internal`, adjusted amounts are considered to be inclusive of tax. If `external`, Paddle calculates the tax and adds it to the amounts provided.
    ///
    /// Only valid for adjustments where the `type` is `partial`.
    ///
    /// If omitted, defaults to `internal`.
    pub fn tax_mode(&mut self, mode: TaxMode) -> &mut Self {
        self.tax_mode = Some(mode);
        self
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<Adjustment> {
        self.client.send(self, Method::POST, "/adjustments").await
    }
}
