//! Unique Paddle IDs

use std::fmt;

use serde::{Deserialize, Serialize};

macro_rules! paddle_id {
    ($(#[$attr:meta])* $name:ident) => {
        $(#[$attr])*
        #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(pub String);

        impl From<String> for $name {
            fn from(value: String) -> Self {
                $name(value)
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                $name(value.to_string())
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
    ($($(#[$attr:meta])* $name:ident,)*) => {
        $(
            paddle_id! {
                $(#[$attr])*
                $name
            }
        )*
    };
}

paddle_id! {
    /// Unique Paddle ID for this address entity, prefixed with `add_`.
    AddressID,

    /// Unique Paddle ID for this customer entity, prefixed with `ctm_`.
    CustomerID,

    /// Unique Paddle ID for this adjustment entity, prefixed with `adj_`.
    AdjustmentID,

    /// Unique Paddle ID for this transaction entity, prefixed with `txn_`.
    TransactionID,

    /// Unique Paddle ID for this subscription entity, prefixed with `sub_`.
    SubscriptionID,

    /// Unique Paddle ID for this transaction item, prefixed with `txnitm_`. Used when working with [adjustments](https://developer.paddle.com/build/transactions/create-transaction-adjustments).
    TransactionItemID,

    /// Unique Paddle ID for this adjustment item, prefixed with `adjitm_`.
    AdjustmentItemID,

    /// Unique Paddle ID for this business entity, prefixed with `biz_`.
    BusinessID,

    /// Unique Paddle ID for this payment method entity, prefixed with `paymtd_`.
    PaymentMethodID,

    /// Unique Paddle ID for this customer portal session entity, prefixed with `cpls_`.
    CustomerPortalSessionID,

    /// Unique Paddle ID for this discount, prefixed with `dsc_`.
    DiscountID,

    /// Unique code that customers can use to apply this discount at checkout. Use letters and numbers only, up to 16 characters. Not case-sensitive.
    DiscountCode,

    /// Unique Paddle ID for this event, prefixed with `evt_`.
    EventID,

    /// Unique Paddle ID for this price, prefixed with `pri_`.
    PriceID,

    /// Unique Paddle ID for this product, prefixed with `pro_`.
    ProductID,

    /// Unique Paddle ID for API keys, prefixed with `apikey_`.
    ApiKeyID,

    /// Unique Paddle ID for payouts, prefixed with `payout_`.
    PayoutID,

    /// Unique Paddle ID for this notification, prefixed with `ntf_`.
    NotificationID,

    /// Unique Paddle ID for this notification setting, prefixed with `ntfset_`.
    NotificationSettingID,

    /// Unique Paddle ID for this notification log, prefixed with `ntflog_`.
    NotificationLogID,

    /// Webhook destination secret key, prefixed with `pdl_ntfset_`. Used for signature verification.
    EndpointSecretKey,

    /// Just a Paddle ID. I've noticed this used in some places.
    PaddleID,

    /// Unique Paddle ID for this simulation event, prefixed with `ntfsimevt_`.
    SimulationEventID,

    /// Unique Paddle ID for this simulation run, prefixed with `ntfsimrun_`.
    SimulationRunID,

    /// Unique Paddle ID for this simulation, prefixed with `ntfsim_`.
    SimulationID,

    /// Paddle ID of the invoice that this transaction is related to, prefixed with `inv_`. Used for compatibility with the Paddle Invoice API, which is now deprecated. This field is scheduled to be removed in the next version of the Paddle API.
    InvoiceId,
}
