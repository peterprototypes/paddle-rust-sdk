//! Unique Paddle IDs

use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Unique Paddle ID for this address entity, prefixed with `add_`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddressID(String);

/// Unique Paddle ID for this customer entity, prefixed with `ctm_`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomerID(String);

/// Unique Paddle ID for this adjustment entity, prefixed with `adj_`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdjustmentID(String);

/// Unique Paddle ID for this transaction entity, prefixed with `txn_`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionID(String);

/// Unique Paddle ID for this subscription entity, prefixed with `sub_`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionID(String);

/// Unique Paddle ID for this transaction item, prefixed with `txnitm_`. Used when working with [adjustments](https://developer.paddle.com/build/transactions/create-transaction-adjustments).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionItemID(String);

/// Unique Paddle ID for this adjustment item, prefixed with `adjitm_`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdjustmentItemID(String);

/// Unique Paddle ID for this business entity, prefixed with `biz_`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BusinessID(String);

/// Unique Paddle ID for this payment method entity, prefixed with `paymtd_`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymentMethodID(String);

/// Unique Paddle ID for this customer portal session entity, prefixed with `cpls_`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomerPortalSessionID(String);

/// Unique Paddle ID for this discount, prefixed with `dsc_`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiscountID(String);

/// Unique code that customers can use to apply this discount at checkout. Use letters and numbers only, up to 16 characters. Not case-sensitive.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiscountCode(String);

/// Unique Paddle ID for this event, prefixed with `evt_`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventID(String);

/// Unique Paddle ID for this price, prefixed with `pri_`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PriceID(String);

/// Unique Paddle ID for this product, prefixed with `pro_`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProductID(pub String);

impl<T: Display> From<T> for ProductID {
    fn from(value: T) -> Self {
        ProductID(value.to_string())
    }
}

// Needed for serialization to comma separated values
impl AsRef<str> for ProductID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Unique Paddle ID for this notification, prefixed with `ntf_`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotificationID(String);

/// Unique Paddle ID for this notification setting, prefixed with `ntfset_`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotificationSettingID(String);

/// Unique Paddle ID for this notification log, prefixed with `ntflog_`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotificationLogID(String);

/// Webhook destination secret key, prefixed with `pdl_ntfset_`. Used for signature verification.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EndpointSecretKey(String);

/// Unique Paddle ID for this entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaddleID(String);

/// Unique Paddle ID for this simulation event, prefixed with `ntfsimevt_`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationEventID(String);

/// Unique Paddle ID for this simulation run, prefixed with `ntfsimrun_`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationRunID(String);

/// Unique Paddle ID for this simulation, prefixed with `ntfsim_`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationID(String);

/// Paddle ID of the invoice that this transaction is related to, prefixed with `inv_`. Used for compatibility with the Paddle Invoice API, which is now deprecated. This field is scheduled to be removed in the next version of the Paddle API.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InvoiceId(String);
