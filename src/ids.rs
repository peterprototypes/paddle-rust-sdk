//! Unique Paddle IDs

use std::fmt::Display;

use serde::{Deserialize, Serialize};

macro_rules! paddle_id {
    ($(#[$attr:meta])* $name:ident) => {
        $(#[$attr])*
        #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(pub String);

        impl<T: Display> From<T> for $name {
            fn from(value: T) -> Self {
                $name(value.to_string())
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }
    };
}

paddle_id! {
    /// Unique Paddle ID for this address entity, prefixed with `add_`.
    AddressID
}

paddle_id! {
    /// Unique Paddle ID for this customer entity, prefixed with `ctm_`.
    CustomerID
}

paddle_id! {
    /// Unique Paddle ID for this adjustment entity, prefixed with `adj_`.
    AdjustmentID
}

paddle_id! {
    /// Unique Paddle ID for this transaction entity, prefixed with `txn_`.
    TransactionID
}
paddle_id! {
    /// Unique Paddle ID for this subscription entity, prefixed with `sub_`.
    SubscriptionID
}

paddle_id! {
    /// Unique Paddle ID for this transaction item, prefixed with `txnitm_`. Used when working with [adjustments](https://developer.paddle.com/build/transactions/create-transaction-adjustments).
    TransactionItemID
}

paddle_id! {
    /// Unique Paddle ID for this adjustment item, prefixed with `adjitm_`.
    AdjustmentItemID
}

paddle_id! {
    /// Unique Paddle ID for this business entity, prefixed with `biz_`.
    BusinessID
}

paddle_id! {
    /// Unique Paddle ID for this payment method entity, prefixed with `paymtd_`.
    PaymentMethodID
}

paddle_id! {
    /// Unique Paddle ID for this customer portal session entity, prefixed with `cpls_`.
    CustomerPortalSessionID
}

paddle_id! {
    /// Unique Paddle ID for this discount, prefixed with `dsc_`.
    DiscountID
}

paddle_id! {
    /// Unique code that customers can use to apply this discount at checkout. Use letters and numbers only, up to 16 characters. Not case-sensitive.
    DiscountCode
}

paddle_id! {
    /// Unique Paddle ID for this event, prefixed with `evt_`.
    EventID
}

paddle_id! {
    /// Unique Paddle ID for this price, prefixed with `pri_`.
    PriceID
}

paddle_id! {
    /// Unique Paddle ID for this product, prefixed with `pro_`.
    ProductID
}

paddle_id! {
    /// Unique Paddle ID for API keys, prefixed with `apikey_`.
    ApiKeyID
}

paddle_id! {
    /// Unique Paddle ID for payouts, prefixed with `payout_`.
    PayoutID
}

paddle_id! {
    /// Unique Paddle ID for this notification, prefixed with `ntf_`.
    NotificationID
}

paddle_id! {
    /// Unique Paddle ID for this notification setting, prefixed with `ntfset_`.
    NotificationSettingID
}

paddle_id! {
    /// Unique Paddle ID for this notification log, prefixed with `ntflog_`.
    NotificationLogID
}

paddle_id! {
    /// Webhook destination secret key, prefixed with `pdl_ntfset_`. Used for signature verification.
    EndpointSecretKey
}

paddle_id! {
    /// Just a Paddle ID. I've noticed this used in some places.
    PaddleID
}

paddle_id! {
    /// Unique Paddle ID for this simulation event, prefixed with `ntfsimevt_`.
    SimulationEventID
}

paddle_id! {
    /// Unique Paddle ID for this simulation run, prefixed with `ntfsimrun_`.
    SimulationRunID
}

paddle_id! {
    /// Unique Paddle ID for this simulation, prefixed with `ntfsim_`.
    SimulationID
}

paddle_id! {
    /// Paddle ID of the invoice that this transaction is related to, prefixed with `inv_`. Used for compatibility with the Paddle Invoice API, which is now deprecated. This field is scheduled to be removed in the next version of the Paddle API.
    InvoiceId
}
