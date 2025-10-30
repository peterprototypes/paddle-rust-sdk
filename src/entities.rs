//! Contains all Paddle entity types.

use std::collections::HashMap;

use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::enums::*;
use crate::ids::*;

/// Import information for this entity. `null` if this entity is not imported.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImportMeta {
    /// Reference or identifier for this entity from the solution where it was imported from.
    pub external_id: Option<String>,
    /// Name of the platform where this entity was imported from.
    pub imported_from: String,
}

/// Represents an address entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Address {
    /// Unique Paddle ID for this address entity, prefixed with `add_`.
    pub id: AddressID,
    /// Unique Paddle ID for this customer entity, prefixed with `ctm_`.
    pub customer_id: CustomerID,
    /// Memorable description for this address.
    pub description: Option<String>,
    /// First line of this address.
    pub first_line: Option<String>,
    /// Second line of this address.
    pub second_line: Option<String>,
    /// City of this address.
    pub city: Option<String>,
    /// ZIP or postal code of this address. Required for some countries.
    pub postal_code: Option<String>,
    /// State, county, or region of this address.
    pub region: Option<String>,
    /// Supported two-letter ISO 3166-1 alpha-2 country code.
    pub country_code: CountryCodeSupported,
    /// Your own structured key-value data.
    pub custom_data: Option<serde_json::Value>,
    /// Whether this entity can be used in Paddle.
    pub status: Status,
    /// RFC 3339 datetime string of when this entity was created. Set automatically by Paddle.
    pub created_at: DateTime<Utc>,
    /// RFC 3339 datetime string of when this entity was updated. Set automatically by Paddle.
    pub updated_at: DateTime<Utc>,
    /// Import information for this entity. `null` if this entity is not imported.
    pub import_meta: Option<ImportMeta>,
}

/// Represents an address entity when previewing addresses.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddressPreview {
    /// ZIP or postal code of this address. Include for more accurate tax calculations.
    pub postal_code: Option<String>,
    /// Supported two-letter ISO 3166-1 alpha-2 country code.
    pub country_code: CountryCodeSupported,
}

/// Breakdown of the total for an adjustment.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdjustmentTotals {
    /// Total before tax. For tax adjustments, the value is 0.
    pub subtotal: String,
    /// Total tax on the subtotal.
    pub tax: String,
    /// Total after tax.
    pub total: String,
    /// Total fee taken by Paddle for this adjustment.
    pub fee: String,
    /// Total earnings. This is the subtotal minus the Paddle fee.
    /// For tax adjustments, this value is negative, which means a positive effect in the transaction earnings.
    /// This is because the fee is originally calculated from the transaction total, so if a tax adjustment is made,
    /// then the fee portion of it is returned.
    pub earnings: String,
    /// Supported three-letter ISO 4217 currency code.
    pub currency_code: CurrencyCode,
}

/// Chargeback fee before conversion to the payout currency. `null` when the chargeback fee is the same as the payout currency.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Original {
    /// Fee amount for this chargeback in the original currency.
    pub amount: String,
    /// Three-letter ISO 4217 currency code for chargeback fees.
    pub currency_code: CurrencyCodeChargebacks,
}

/// Chargeback fees incurred for this adjustment. Only returned when the adjustment `action` is `chargeback` or `chargeback_warning`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChargebackFee {
    /// Chargeback fee converted into the payout currency.
    pub amount: String,
    /// Chargeback fee before conversion to the payout currency. `null` when the chargeback fee is the same as the payout currency.
    pub original: Option<Original>,
}

/// Breakdown of how this adjustment affects your payout balance.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdjustmentPayoutTotals {
    /// Adjustment total before tax and fees.
    pub subtotal: String,
    /// Total tax on the adjustment subtotal.
    pub tax: String,
    /// Adjustment total after tax.
    pub total: String,
    /// Adjusted Paddle fee.
    pub fee: String,
    /// Chargeback fees incurred for this adjustment. Only returned when the adjustment `action` is `chargeback` or `chargeback_warning`.
    pub chargeback_fee: Option<ChargebackFee>,
    /// Adjusted payout earnings. This is the adjustment total plus adjusted Paddle fees, excluding chargeback fees.
    pub earnings: String,
    /// Supported three-letter ISO 4217 currency code for payouts from Paddle.
    pub currency_code: CurrencyCodePayouts,
}

/// Calculated totals for the tax applied to this adjustment.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdjustmentTaxRateUsedTotals {
    /// Total before tax. For tax adjustments, the value is 0.
    pub subtotal: String,
    /// Total tax on the subtotal.
    pub tax: String,
    /// Total after tax.
    pub total: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdjustmentTaxRateUsed {
    /// Rate used to calculate tax for this adjustment.
    pub tax_rate: String,
    /// Calculated totals for the tax applied to this adjustment.
    pub totals: AdjustmentTaxRateUsedTotals,
}

/// Represents an adjustment entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Adjustment {
    /// Unique Paddle ID for this adjustment entity, prefixed with `adj_`.
    pub id: AdjustmentID,
    /// How this adjustment impacts the related transaction.
    pub action: AdjustmentAction,
    /// Type of adjustment. Use `full` to adjust the grand total for the related transaction. Include an `items` array when creating a `partial` adjustment. If omitted, defaults to `partial`.
    pub r#type: AdjustmentType,
    /// Unique Paddle ID for this transaction entity, prefixed with `txn_`.
    pub transaction_id: TransactionID,
    /// Paddle ID for the subscription related to this adjustment, prefixed with `sub_`.
    /// Set automatically by Paddle based on the `subscription_id` of the related transaction.
    pub subscription_id: Option<SubscriptionID>,
    /// Unique Paddle ID for this customer entity, prefixed with `ctm_`.
    pub customer_id: CustomerID,
    /// Why this adjustment was created. Appears in the Paddle dashboard. Retained for record-keeping purposes.
    pub reason: String,
    /// Whether this adjustment was applied to the related customer's credit balance. Only returned for `credit` adjustments.
    pub credit_applied_to_balance: Option<bool>,
    /// Supported three-letter ISO 4217 currency code.
    pub currency_code: CurrencyCode,
    /// Status of this adjustment. Set automatically by Paddle.
    ///
    /// Most refunds for live accounts are created with the status of `pending_approval` until reviewed by Paddle, but some are automatically approved. For sandbox accounts, Paddle automatically approves refunds every ten minutes.
    ///
    /// Credit adjustments don't require approval from Paddle, so they're created as `approved`.
    pub status: AdjustmentStatus,
    /// List of items on this adjustment. Required if `type` is not populated or set to `partial`.
    pub items: Vec<AdjustmentItem>,
    /// Breakdown of the total for an adjustment.
    pub totals: AdjustmentTotals,
    /// Breakdown of how this adjustment affects your payout balance.
    pub payout_totals: Option<AdjustmentPayoutTotals>,
    /// List of tax rates applied for this adjustment.
    #[serde(default)]
    pub tax_rates_used: Vec<AdjustmentTaxRateUsed>,
    /// RFC 3339 datetime string of when this entity was created. Set automatically by Paddle.
    pub created_at: DateTime<Utc>,
    /// RFC 3339 datetime string of when this entity was updated. Set automatically by Paddle.
    pub updated_at: DateTime<Utc>,
}

/// Represents an adjustment entity when creating adjustments.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdjustmentCreate {
    /// Unique Paddle ID for this adjustment entity, prefixed with `adj_`.
    pub id: AdjustmentID,
    /// How this adjustment impacts the related transaction.
    pub action: AdjustmentAction,
    /// Type of adjustment. Use `full` to adjust the grand total for the related transaction. Include an `items` array when creating a `partial` adjustment. If omitted, defaults to `partial`.
    pub r#type: AdjustmentType,
    /// Unique Paddle ID for this transaction entity, prefixed with `txn_`.
    pub transaction_id: TransactionID,
    /// Paddle ID for the subscription related to this adjustment, prefixed with `sub_`.
    /// Set automatically by Paddle based on the `subscription_id` of the related transaction.
    pub subscription_id: SubscriptionID,
    /// Unique Paddle ID for this customer entity, prefixed with `ctm_`.
    pub customer_id: CustomerID,
    /// Why this adjustment was created. Appears in the Paddle dashboard. Retained for recordkeeping purposes.
    pub reason: String,
    /// Whether this adjustment was applied to the related customer's credit balance. Only returned for `credit` adjustments.
    pub credit_applied_to_balance: Option<bool>,
    /// Supported three-letter ISO 4217 currency code.
    pub currency_code: CurrencyCode,
    /// Status of this adjustment. Set automatically by Paddle.
    ///
    /// Most refunds for live accounts are created with the status of `pending_approval` until reviewed by Paddle, but some are automatically approved. For sandbox accounts, Paddle automatically approves refunds every ten minutes.
    ///
    /// Credit adjustments don't require approval from Paddle, so they're created as `approved`.
    pub status: AdjustmentStatus,
    /// List of transaction items to adjust. Required if `type` is not populated or set to `partial`.
    pub items: Option<Vec<AdjustmentItem>>,
    /// Breakdown of the total for an adjustment.
    pub totals: AdjustmentTotals,
    /// Breakdown of how this adjustment affects your payout balance.
    pub payout_totals: Option<AdjustmentPayoutTotals>,
    /// RFC 3339 datetime string of when this entity was created. Set automatically by Paddle.
    pub created_at: String,
    /// RFC 3339 datetime string of when this entity was updated. Set automatically by Paddle.
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimePeriod {
    /// RFC 3339 datetime string.
    pub starts_at: DateTime<Utc>,
    /// RFC 3339 datetime string.
    pub ends_at: DateTime<Utc>,
}

/// How proration was calculated for this item. Populated when a transaction is created from a subscription change, where `proration_billing_mode` was `prorated_immediately` or `prorated_next_billing_period`. Set automatically by Paddle.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Proration {
    /// Rate used to calculate proration.
    pub rate: String,
    pub billing_period: TimePeriod,
}

/// Breakdown of the total for an adjustment item.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdjustmentItemTotals {
    /// Amount multiplied by quantity.
    pub subtotal: String,
    /// Total tax on the subtotal.
    pub tax: String,
    /// Total after tax.
    pub total: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdjustmentItem {
    /// Unique Paddle ID for this transaction item, prefixed with `txnitm_`. Used when working with [adjustments](https://developer.paddle.com/build/transactions/create-transaction-adjustments).
    pub item_id: TransactionItemID,
    /// Type of adjustment for this transaction item. `tax` adjustments are automatically created by Paddle.
    /// Include `amount` when creating a `partial` adjustment.
    pub r#type: AdjustmentItemType,
    /// Amount adjusted for this transaction item. Required when item type is `partial`.
    pub amount: Option<String>,
    /// How proration was calculated for this adjustment item.
    pub proration: Option<Proration>,
    /// Breakdown of the total for an adjustment item.
    pub totals: AdjustmentItemTotals,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdjustmentItemInput {
    /// Unique Paddle ID for this transaction item, prefixed with `txnitm_`. Used when working with [adjustments](https://developer.paddle.com/build/transactions/create-transaction-adjustments).
    pub item_id: TransactionItemID,
    /// Type of adjustment for this transaction item. `tax` adjustments are automatically created by Paddle.
    /// Include `amount` when creating a `partial` adjustment.
    pub r#type: AdjustmentItemType,
    /// Amount adjusted for this transaction item. Required when item type is `partial`.
    pub amount: Option<String>,
}

/// Represents an adjustment entity when previewing adjustments.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdjustmentPreview {
    /// Unique Paddle ID for this transaction entity, prefixed with `txn_`.
    pub transaction_id: TransactionID,
    /// List of transaction items that this adjustment is for.
    pub items: Vec<AdjustmentItem>,
    /// Breakdown of the total for an adjustment.
    pub totals: AdjustmentTotals,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Duration {
    /// Unit of time.
    pub interval: Interval,
    /// Amount of time.
    pub frequency: u64,
}

/// Details for invoicing. Required if `collection_mode` is `manual`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BillingDetails {
    /// Whether the related transaction may be paid using Paddle Checkout. If omitted when creating a transaction, defaults to `false`.
    pub enable_checkout: bool,
    /// Customer purchase order number. Appears on invoice documents.
    pub purchase_order_number: String,
    /// Notes or other information to include on this invoice. Appears on invoice documents.
    pub additional_information: Option<String>,
    pub payment_terms: Duration,
}

/// Details for invoicing. Required if `collection_mode` is `manual`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BillingDetailsUpdate {
    /// Whether the related transaction may be paid using Paddle Checkout.
    pub enable_checkout: bool,
    /// Customer purchase order number. Appears on invoice documents.
    pub purchase_order_number: String,
    /// Notes or other information to include on this invoice. Appears on invoice documents.
    pub additional_information: Option<String>,
    pub payment_terms: Duration,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Contact {
    /// Full name.
    pub name: String,
    /// Email address for this entity.
    pub email: String,
}

/// Represents a business entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Business {
    /// Unique Paddle ID for this business entity, prefixed with `biz_`.
    pub id: BusinessID,
    /// Unique Paddle ID for this customer entity, prefixed with `ctm_`.
    pub customer_id: CustomerID,
    /// Full name.
    pub name: String,
    /// Company number for this business.
    pub company_number: Option<String>,
    /// Tax or VAT Number for this business.
    pub tax_identifier: Option<String>,
    /// Whether this entity can be used in Paddle.
    pub status: Status,
    /// List of contacts related to this business, typically used for sending invoices.
    pub contacts: Option<Vec<Contact>>,
    /// RFC 3339 datetime string of when this entity was created. Set automatically by Paddle.
    pub created_at: DateTime<Utc>,
    /// RFC 3339 datetime string of when this entity was updated. Set automatically by Paddle.
    pub updated_at: DateTime<Utc>,
    /// Your own structured key-value data.
    pub custom_data: Option<serde_json::Value>,
    /// Import information for this entity. `null` if this entity is not imported.
    pub import_meta: Option<ImportMeta>,
}

/// Card metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Card {
    /// Type of credit or debit card used to pay.
    pub r#type: CardType,
    /// Last four digits of the card used to pay.
    pub last4: String,
    /// Month of the expiry date of the card used to pay.
    pub expiry_month: i64,
    /// Year of the expiry date of the card used to pay.
    pub expiry_year: i64,
    /// The name on the card used to pay.
    pub cardholder_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomerBalance {
    /// Total amount of credit available to use.
    pub available: String,
    /// Total amount of credit temporarily reserved for `billed` transactions.
    pub reserved: String,
    /// Total amount of credit used.
    pub used: String,
}

/// Represents a credit balance for a customer.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreditBalance {
    /// Unique Paddle ID for this customer entity, prefixed with `ctm_`.
    pub customer_id: CustomerID,
    /// Supported three-letter ISO 4217 currency code.
    pub currency_code: CurrencyCode,
    pub balance: CustomerBalance,
}

/// Represents a customer entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Customer {
    /// Unique Paddle ID for this customer entity, prefixed with `ctm_`.
    pub id: CustomerID,
    /// Full name of this customer. Required when creating transactions where `collection_mode` is `manual` (invoices).
    pub name: Option<String>,
    /// Email address for this entity.
    pub email: String,
    /// Whether this customer opted into marketing from you. `false` unless customers check the marketing consent box
    /// when using Paddle Checkout. Set automatically by Paddle.
    pub marketing_consent: bool,
    /// Whether this entity can be used in Paddle.
    pub status: Status,
    /// Your own structured key-value data.
    pub custom_data: Option<serde_json::Value>,
    /// Valid IETF BCP 47 short form locale tag. If omitted, defaults to `en`.
    pub locale: String,
    /// RFC 3339 datetime string of when this entity was created. Set automatically by Paddle.
    pub created_at: DateTime<Utc>,
    /// RFC 3339 datetime string of when this entity was updated. Set automatically by Paddle.
    pub updated_at: DateTime<Utc>,
    /// Import information for this entity. `null` if this entity is not imported.
    pub import_meta: Option<ImportMeta>,
}

/// PayPal metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PayPal {
    /// Email address associated with the PayPal account.
    pub email: String,
    /// PayPal payment method identifier.
    pub reference: String,
}

/// Represents a customer payment method entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymentMethod {
    /// Unique Paddle ID for this payment method entity, prefixed with `paymtd_`.
    pub id: PaymentMethodID,
    /// Unique Paddle ID for this customer entity, prefixed with `ctm_`.
    pub customer_id: CustomerID,
    /// Unique Paddle ID for this address entity, prefixed with `add_`.
    pub address_id: AddressID,
    /// Type of payment method saved.
    pub r#type: SavedPaymentMethodType,
    /// Information about the credit or debit card saved. `null` unless `type` is `card`.
    pub card: Option<Card>,
    /// Information about the PayPal payment method saved. `null` unless `type` is `paypal`.
    pub paypal: Option<PayPal>,
    /// Describes how this payment method was saved.
    pub origin: PaymentMethodOrigin,
    /// RFC 3339 datetime string of when this entity was saved. Set automatically by Paddle.
    pub saved_at: DateTime<Utc>,
    /// RFC 3339 datetime string of when this entity was updated. Set automatically by Paddle.
    pub updated_at: DateTime<Utc>,
}

/// Authenticated customer portal deep links that aren't associated with a specific entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomerPortalSessionGeneralUrls {
    /// Link to the overview page in the customer portal.
    pub overview: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomerPortalSessionSubscriptionUrls {
    /// Unique Paddle ID for this subscription entity, prefixed with `sub_`.
    pub id: SubscriptionID,
    /// Link to the page for this subscription in the customer portal with the subscription cancellation form pre-opened. Use as part of cancel subscription workflows.
    pub cancel_subscription: String,
    /// Link to the page for this subscription in the customer portal with the payment method update form pre-opened. Use as part of workflows to let customers update their payment details.
    ///
    /// If a manually-collected subscription, opens the overview page for this subscription.
    pub update_subscription_payment_method: String,
}

/// Authenticated customer portal deep links. For security, the `token` appended to each link is temporary. You shouldn't store these links.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomerPortalSessionUrls {
    /// Authenticated customer portal deep links that aren't associated with a specific entity.
    pub general: CustomerPortalSessionGeneralUrls,
    /// List of generated authenticated customer portal deep links for the subscriptions passed in the `subscription_ids` array in the request.
    ///
    /// If subscriptions are paused or canceled, links open the overview page for a subscription.
    ///
    /// Empty if no subscriptions passed in the request.
    pub subscriptions: Vec<CustomerPortalSessionSubscriptionUrls>,
}

/// Represents a customer portal session.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomerPortalSession {
    /// Unique Paddle ID for this customer portal session entity, prefixed with `cpls_`.
    pub id: CustomerPortalSessionID,
    /// Unique Paddle ID for this customer entity, prefixed with `ctm_`.
    pub customer_id: CustomerID,
    /// Authenticated customer portal deep links. For security, the `token` appended to each link is temporary. You shouldn't store these links.
    pub urls: CustomerPortalSessionUrls,
    /// RFC 3339 datetime string.
    pub created_at: DateTime<Utc>,
}

/// Represents a customer authentication token.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomerAuthenticationToken {
    /// Authentication token generated by Paddle for this customer. Pass to Paddle.js when opening a checkout to let customers work with saved payment methods.
    pub customer_auth_token: String,
    /// RFC 3339 datetime string.
    pub expires_at: DateTime<Utc>,
}

/// Represents a discount entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Discount {
    /// Unique Paddle ID for this discount, prefixed with `dsc_`.
    pub id: DiscountID,
    /// Whether this entity can be used in Paddle.
    pub status: DiscountStatus,
    /// Short description for this discount for your reference. Not shown to customers.
    pub description: String,
    /// Whether this discount can be redeemed by customers at checkout (`true`) or not (`false`).
    pub enabled_for_checkout: bool,
    /// Unique code that customers can use to redeem this discount at checkout. Not case-sensitive.
    pub code: Option<String>,
    /// Type of discount. Determines how this discount impacts the checkout or transaction total.
    pub r#type: DiscountType,
    /// Amount to discount by. For `percentage` discounts, must be an amount between `0.01` and `100`. For `flat` and `flat_per_seat` discounts, amount in the lowest denomination for a currency.
    pub amount: String,
    /// Supported three-letter ISO 4217 currency code. Required where discount type is `flat` or `flat_per_seat`.
    pub currency_code: Option<CurrencyCode>,
    /// Whether this discount applies for multiple subscription billing periods (`true`) or not (`false`).
    pub recur: bool,
    /// Number of subscription billing periods that this discount recurs for. Requires `recur`. `null` if this discount recurs forever.
    ///
    /// Subscription renewals, midcycle changes, and one-time charges billed to a subscription aren't considered a redemption. `times_used` is not incremented in these cases.
    pub maximum_recurring_intervals: Option<i64>,
    /// Maximum number of times this discount can be redeemed. This is an overall limit for this discount, rather than a per-customer limit. `null` if this discount can be redeemed an unlimited amount of times.
    ///
    /// Paddle counts a usage as a redemption on a checkout, transaction, or the initial application against a subscription. Transactions created for subscription renewals, midcycle changes, and one-time charges aren't considered a redemption.
    pub usage_limit: Option<i64>,
    /// Product or price IDs that this discount is for. When including a product ID, all prices for that product can be discounted. `null` if this discount applies to all products and prices.
    pub restrict_to: Option<Vec<String>>,
    /// RFC 3339 datetime string of when this discount expires. Discount can no longer be redeemed after this date has elapsed. `null` if this discount can be redeemed forever.
    ///
    /// Expired discounts can't be redeemed against transactions or checkouts, but can be applied when updating subscriptions.
    pub expires_at: Option<DateTime<Utc>>,
    /// Your own structured key-value data.
    pub custom_data: Option<serde_json::Value>,
    /// How many times this discount has been redeemed. Automatically incremented by Paddle.
    ///
    /// Paddle counts a usage as a redemption on a checkout, transaction, or subscription. Transactions created for subscription renewals, midcycle changes, and one-time charges aren't considered a redemption.
    #[serde(default)]
    pub times_used: i64,
    /// RFC 3339 datetime string of when this entity was created. Set automatically by Paddle.
    pub created_at: DateTime<Utc>,
    /// RFC 3339 datetime string of when this entity was updated. Set automatically by Paddle.
    pub updated_at: DateTime<Utc>,
    /// Import information for this entity. `null` if this entity is not imported.
    pub import_meta: Option<ImportMeta>,
}

/// Details of the discount applied to this subscription.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionDiscountTimePeriod {
    /// Unique Paddle ID for this discount, prefixed with `dsc_`.
    pub id: DiscountID,
    /// RFC 3339 datetime string of when this discount was first applied. `null` for canceled subscriptions where a discount was redeemed but never applied to a transaction.
    pub starts_at: Option<DateTime<FixedOffset>>,
    /// RFC 3339 datetime string of when this discount no longer applies. Where a discount has `maximum_recurring_intervals`, this is the date of the last billing period where this discount applies. `null` where a discount recurs forever.
    pub ends_at: Option<DateTime<FixedOffset>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Errors {
    /// Field where validation error occurred.
    pub field: String,
    /// Information about how the field failed validation.
    pub message: String,
}

/// Represents an error.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Error {
    /// Type of error encountered.
    pub r#type: Type,
    /// Short snake case string that describes this error. Use to search the error reference.
    pub code: String,
    /// Some information about what went wrong as a human-readable string.
    pub detail: String,
    /// Link to a page in the error reference for this specific error.
    pub documentation_url: String,
    /// List of validation errors. Only returned when there's a validation error.
    pub errors: Vec<Errors>,
}

/// Information about this response.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Meta {
    /// Unique ID for the request relating to this response. Provide this when contacting Paddle support about a specific request.
    pub request_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidationError {
    /// Represents an error.
    pub error: Error,
    /// Information about this response.
    pub meta: Meta,
}

/// Represents an event entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    /// Unique Paddle ID for this event, prefixed with `evt_`.
    pub event_id: EventID,
    /// RFC 3339 datetime string.
    pub occurred_at: DateTime<Utc>,
    /// New or changed entity.
    #[serde(flatten)]
    pub data: EventData,
}

/// Represents an event type.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventType {
    /// Type of event sent by Paddle, in the format `entity.event_type`.
    pub name: String, // EventTypeName,
    /// Short description of this event type.
    pub description: String,
    /// Group for this event type. Typically the entity that this event relates to.
    pub group: String,
    /// List of API versions that this event type supports.
    pub available_versions: Vec<u64>,
}

/// A base representation of monetary value unformatted in the lowest denomination with currency code.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Money {
    /// Amount in the lowest denomination for the currency, e.g. 10 USD = 1000 (cents). Although represented as a string, this value must be a valid integer.
    pub amount: String,
    /// Supported three-letter ISO 4217 currency code.
    pub currency_code: CurrencyCode,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnitPriceOverride {
    /// Supported two-letter ISO 3166-1 alpha-2 country code. Customers located in the listed countries are charged the override price.
    pub country_codes: Vec<CountryCodeSupported>,
    /// A base representation of monetary value unformatted in the lowest denomination with currency code.
    pub unit_price: Money,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PriceQuantity {
    /// Minimum quantity of the product related to this price that can be bought. Required if `maximum` set.
    pub minimum: u64,
    /// Maximum quantity of the product related to this price that can be bought. Required if `minimum` set. Must be greater than or equal to the `minimum` value.
    pub maximum: u64,
}

/// Represents a price entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Price {
    /// Unique Paddle ID for this price, prefixed with `pri_`.
    pub id: PriceID,
    /// Unique Paddle ID for this product, prefixed with `pro_`.
    pub product_id: ProductID,
    /// Internal description for this price, not shown to customers. Typically notes for your team.
    pub description: String,
    /// Type of item. Standard items are considered part of your catalog and are shown on the Paddle dashboard.
    pub r#type: CatalogType,
    /// Name of this price, shown to customers at checkout and on invoices. Typically describes how often the related product bills.
    pub name: Option<String>,
    /// How often this price should be charged. `null` if price is non-recurring (one-time).
    pub billing_cycle: Option<Duration>,
    /// Trial period for the product related to this price. The billing cycle begins once the trial period is over. `null` for no trial period. Requires `billing_cycle`.
    pub trial_period: Option<Duration>,
    /// How tax is calculated for this price.
    pub tax_mode: TaxMode,
    /// A base representation of monetary value unformatted in the lowest denomination with currency code.
    pub unit_price: Money,
    /// List of unit price overrides. Use to override the base price with a custom price and currency for a country or group of countries.
    #[serde(default)]
    pub unit_price_overrides: Vec<UnitPriceOverride>,
    pub quantity: PriceQuantity,
    /// Whether this entity can be used in Paddle.
    pub status: Status,
    /// Your own structured key-value data.
    pub custom_data: Option<serde_json::Value>,
    /// Import information for this entity. `null` if this entity is not imported.
    pub import_meta: Option<ImportMeta>,
    /// RFC 3339 datetime string of when this entity was created. Set automatically by Paddle.
    pub created_at: DateTime<Utc>,
    /// RFC 3339 datetime string of when this entity was updated. Set automatically by Paddle.
    pub updated_at: DateTime<Utc>,
}

/// Represents a product entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Product {
    /// Unique Paddle ID for this product, prefixed with `pro_`.
    pub id: ProductID,
    /// Name of this product.
    pub name: String,
    /// Short description for this product.
    pub description: Option<String>,
    /// Type of item. Standard items are considered part of your catalog and are shown on the Paddle dashboard.
    pub r#type: CatalogType,
    /// Tax category for this product. Used for charging the correct rate of tax. Selected tax category must be enabled on your Paddle account.
    pub tax_category: TaxCategory,
    /// Image for this product. Included in the checkout and on some customer documents.
    pub image_url: Option<String>,
    /// Your own structured key-value data.
    pub custom_data: Option<serde_json::Value>,
    /// Whether this entity can be used in Paddle.
    pub status: Status,
    /// Import information for this entity. `null` if this entity is not imported.
    pub import_meta: Option<ImportMeta>,
    /// RFC 3339 datetime string of when this entity was created. Set automatically by Paddle.
    pub created_at: DateTime<Utc>,
    /// RFC 3339 datetime string of when this entity was updated. Set automatically by Paddle.
    pub updated_at: DateTime<Utc>,
}

/// Represents a subscription item.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionItem {
    /// Status of this subscription item. Set automatically by Paddle.
    pub status: SubscriptionItemStatus,
    /// Quantity of this item on the subscription.
    pub quantity: i64,
    /// Whether this is a recurring item. `false` if one-time.
    pub recurring: bool,
    /// RFC 3339 datetime string of when this entity was created. Set automatically by Paddle.
    pub created_at: DateTime<Utc>,
    /// RFC 3339 datetime string of when this entity was updated. Set automatically by Paddle.
    pub updated_at: DateTime<Utc>,
    /// RFC 3339 datetime string of when this item was last billed.
    pub previously_billed_at: Option<DateTime<FixedOffset>>,
    /// RFC 3339 datetime string of when this item is next scheduled to be billed.
    pub next_billed_at: Option<DateTime<FixedOffset>>,
    /// Trial dates for this item.
    pub trial_dates: Option<TimePeriod>,
    /// Represents a price entity.
    pub price: Price,
    /// Represents a product entity.
    pub product: Product,
}

/// Keys used for working with paginated results.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pagination {
    /// Number of entities per page for this response. May differ from the number requested if the requested number is greater than the maximum.
    pub per_page: i64,
    /// URL containing the query parameters of the original request, along with the `after` parameter that marks the starting point of the next page. Always returned, even if `has_more` is `false`.
    pub next: String,
    /// Whether this response has another page.
    pub has_more: bool,
    /// Estimated number of entities for this response.
    pub estimated_total: i64,
}

/// Information about this response.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MetaPaginated {
    /// Unique ID for the request relating to this response. Provide this when contacting Paddle support about a specific request.
    pub request_id: String,
    /// Keys used for working with paginated results.
    pub pagination: Pagination,
}

/// Information about the payment method used for a payment attempt.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MethodDetails {
    /// Type of payment method used for this payment attempt.
    pub r#type: PaymentMethodType,
    /// Information about the credit or debit card used to pay. `null` unless `type` is `card`.
    pub card: Option<Card>,
}

/// Notification payload. Includes the new or changed event.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotificationPayload {
    /// Unique Paddle ID for this notification, prefixed with `ntf_`.
    pub notification_id: NotificationID,
    /// Unique Paddle ID for this event, prefixed with `evt_`.
    pub event_id: EventID,
    /// Type of event sent by Paddle, in the format `entity.event_type`.
    pub event_type: EventTypeName,
    /// RFC 3339 datetime string.
    pub occurred_at: String,
    /// New or changed entity.
    pub data: HashMap<String, String>,
}

/// Represents a notification entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Notification {
    /// Unique Paddle ID for this notification, prefixed with `ntf_`.
    pub id: NotificationID,
    /// Type of event sent by Paddle, in the format `entity.event_type`.
    pub r#type: EventTypeName,
    /// Status of this notification.
    pub status: NotificationStatus,
    pub payload: NotificationPayload,
    /// RFC 3339 datetime string.
    pub occurred_at: DateTime<FixedOffset>,
    /// RFC 3339 datetime string of when this notification was delivered. `null` if not yet delivered successfully.
    pub delivered_at: Option<DateTime<FixedOffset>>,
    /// RFC 3339 datetime string of when this notification was replayed. `null` if not replayed.
    pub replayed_at: Option<DateTime<FixedOffset>>,
    /// Describes how this notification was created.
    pub origin: NotificationOrigin,
    /// RFC 3339 datetime string of when this notification was last attempted.
    pub last_attempt_at: Option<DateTime<FixedOffset>>,
    /// RFC 3339 datetime string of when this notification is scheduled to be retried.
    pub retry_at: Option<DateTime<FixedOffset>>,
    /// How many times delivery of this notification has been attempted. Automatically incremented by Paddle after an attempt.
    pub times_attempted: i64,
    /// Unique Paddle ID for this notification setting, prefixed with `ntfset_`.
    pub notification_setting_id: NotificationSettingID,
}

/// Represents a notification log entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotificationLog {
    /// Unique Paddle ID for this notification log, prefixed with `ntflog_`.
    pub id: NotificationLogID,
    /// HTTP code sent by the responding server.
    pub response_code: i64,
    /// Content-Type sent by the responding server.
    pub response_content_type: Option<String>,
    /// Response body sent by the responding server. Typically empty for success responses.
    pub response_body: String,
    /// RFC 3339 datetime string.
    pub attempted_at: String,
}

/// Represents a notification destination.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotificationSetting {
    /// Unique Paddle ID for this notification setting, prefixed with `ntfset_`.
    pub id: NotificationSettingID,
    /// Short description for this notification destination. Shown in the Paddle dashboard.
    pub description: String,
    /// Where notifications should be sent for this destination.
    pub r#type: NotificationSettingType,
    /// Webhook endpoint URL or email address.
    pub destination: String,
    /// Whether Paddle should try to deliver events to this notification destination.
    pub active: bool,
    /// API version that returned objects for events should conform to. Must be a valid version of the Paddle API. Can't be a version older than your account default.
    pub api_version: i64,
    /// Whether potentially sensitive fields should be sent to this notification destination.
    pub include_sensitive_fields: bool,
    /// Subscribed events for this notification destination.
    pub subscribed_events: Vec<EventType>,
    /// Webhook destination secret key, prefixed with `pdl_ntfset_`. Used for signature verification.
    pub endpoint_secret_key: EndpointSecretKey,
    /// Whether Paddle should deliver real platform events, simulation events or both to this notification destination.
    pub traffic_source: TrafficSource,
}

/// Represents a notification destination when creating notification destinations.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotificationSettingCreate {
    /// Unique Paddle ID for this notification setting, prefixed with `ntfset_`.
    pub id: NotificationSettingID,
    /// Short description for this notification destination. Shown in the Paddle Dashboard.
    pub description: String,
    /// Where notifications should be sent for this destination.
    pub r#type: NotificationSettingType,
    /// Webhook endpoint URL or email address.
    pub destination: String,
    /// Whether Paddle should try to deliver events to this notification destination.
    pub active: bool,
    /// API version that returned objects for events should conform to. Must be a valid version of the Paddle API. Can't be a version older than your account default. If omitted, defaults to your account default version.
    pub api_version: i64,
    /// Whether potentially sensitive fields should be sent to this notification destination. If omitted, defaults to `false`.
    pub include_sensitive_fields: bool,
    /// Subscribed events for this notification destination. When creating or updating a notification destination, pass an array of event type names only. Paddle returns the complete event type object.
    pub subscribed_events: Vec<String>,
    /// Webhook destination secret key, prefixed with `pdl_ntfset_`. Used for signature verification.
    pub endpoint_secret_key: EndpointSecretKey,
    /// Whether Paddle should deliver real platform events, simulation events or both to this notification destination. If omitted, defaults to `platform`.
    pub traffic_source: TrafficSource,
}

/// Represents a notification destination when updating notification destinations.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotificationSettingUpdate {
    /// Short description for this notification destination. Shown in the Paddle Dashboard.
    pub description: String,
    /// Webhook endpoint URL or email address.
    pub destination: String,
    /// Whether Paddle should try to deliver events to this notification destination.
    pub active: bool,
    /// API version that returned objects for events should conform to. Must be a valid version of the Paddle API. Can't be a version older than your account default. Defaults to your account default if omitted.
    pub api_version: i64,
    /// Whether potentially sensitive fields should be sent to this notification destination.
    pub include_sensitive_fields: bool,
    /// Subscribed events for this notification destination. When creating or updating a notification destination, pass an array of event type names only. Paddle returns the complete event type object.
    pub subscribed_events: Option<Vec<String>>,
    /// Whether Paddle should deliver real platform events, simulation events or both to this notification destination.
    pub traffic_source: TrafficSource,
}

/// Represents a price preview entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PricePreview {
    /// Unique Paddle ID for this price, prefixed with `pri_`.
    /// The value is null for custom prices being previewed.
    pub id: Option<PriceID>,
    /// Paddle ID for the product that this price is for, prefixed with `pro_`.
    /// The value is null for custom products being previewed.
    pub product_id: Option<ProductID>,
    /// Internal description for this price, not shown to customers. Typically notes for your team.
    pub description: String,
    /// Type of item. Standard items are considered part of your catalog and are shown on the Paddle dashboard.
    pub r#type: CatalogType,
    /// Name of this price, shown to customers at checkout and on invoices. Typically describes how often the related product bills.
    pub name: Option<String>,
    /// How often this price should be charged. `null` if price is non-recurring (one-time).
    pub billing_cycle: Option<Duration>,
    /// Trial period for the product related to this price. The billing cycle begins once the trial period is over. `null` for no trial period. Requires `billing_cycle`.
    pub trial_period: Option<Duration>,
    /// How tax is calculated for this price.
    pub tax_mode: TaxMode,
    /// A base representation of monetary value unformatted in the lowest denomination with currency code.
    pub unit_price: Money,
    /// List of unit price overrides. Use to override the base price with a custom price and currency for a country or group of countries.
    pub unit_price_overrides: Vec<UnitPriceOverride>,
    pub quantity: PriceQuantity,
    /// Whether this entity can be used in Paddle.
    pub status: Status,
    /// Your own structured key-value data.
    pub custom_data: Option<serde_json::Value>,
    /// Import information for this entity. `null` if this entity is not imported.
    pub import_meta: ImportMeta,
    /// RFC 3339 datetime string of when this entity was created. Set automatically by Paddle.
    pub created_at: String,
    /// RFC 3339 datetime string of when this entity was updated. Set automatically by Paddle.
    pub updated_at: String,
}

/// Represents a product (preview) entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProductPreview {
    /// Unique Paddle ID for this product, prefixed with `pro_`.
    /// The value is null for custom products being previewed.
    pub id: Option<ProductID>,
    /// Name of this product.
    pub name: String,
    /// Short description for this product.
    pub description: Option<String>,
    /// Type of item. Standard items are considered part of your catalog and are shown on the Paddle dashboard.
    pub r#type: CatalogType,
    /// Tax category for this product. Used for charging the correct rate of tax. Selected tax category must be enabled on your Paddle account.
    pub tax_category: TaxCategory,
    /// Image for this product. Included in the checkout and on some customer documents.
    pub image_url: Option<String>,
    /// Your own structured key-value data.
    pub custom_data: Option<serde_json::Value>,
    /// Whether this entity can be used in Paddle.
    pub status: Status,
    /// Import information for this entity. `null` if this entity is not imported.
    pub import_meta: Option<ImportMeta>,
    /// RFC 3339 datetime string of when this entity was created. Set automatically by Paddle.
    pub created_at: DateTime<Utc>,
    /// RFC 3339 datetime string of when this entity was updated. Set automatically by Paddle.
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReportFilterValue {
    String(String),
    Array(Vec<String>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReportFilter<T: Serialize> {
    /// Field name to filter by.
    pub name: T,
    /// Operator to use when filtering. Valid when filtering by `updated_at`, `null` otherwise.
    pub operator: Option<FilterOperator>,
    /// Value to filter by. Check the allowed values descriptions for the `name` field to see valid values for a field.
    pub value: ReportFilterValue,
}

/// Represents a report entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReportBase {
    /// Unique Paddle ID for this entity.
    pub id: PaddleID,
    pub r#type: String,
    pub filters: Vec<serde_json::Value>,
    /// Status of this report. Set automatically by Paddle.
    ///
    /// Reports are created as `pending` initially, then move to `ready` when they're available to download.
    pub status: ReportStatus,
    /// Number of records in this report. `null` if the report is `pending`.
    pub rows: Option<i64>,
    /// RFC 3339 datetime string of when this report expires. The report is no longer available to download after this date.
    pub expires_at: Option<DateTime<Utc>>,
    /// RFC 3339 datetime string of when this entity was updated. Set automatically by Paddle.
    pub updated_at: DateTime<Utc>,
    /// RFC 3339 datetime string of when this entity was created. Set automatically by Paddle.
    pub created_at: DateTime<Utc>,
}

/// Information about the request. Sent by Paddle as part of the simulation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationEventRequest {
    /// Request body sent by Paddle.
    pub body: String,
}

/// Information about the response. Sent by the responding server for the notification setting.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationEventResponse {
    /// Response body sent by the responding server. May be empty for success responses.
    pub body: String,
    /// HTTP status code sent by the responding server.
    pub status_code: i64,
}

/// Represents a simulation event.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationEvent {
    /// Unique Paddle ID for this simulation event, prefixed with `ntfsimevt_`.
    pub id: SimulationEventID,
    /// Status of this simulation run log.
    pub status: SimulationEventStatus,
    /// Type of event sent by Paddle, in the format `entity.event_type`.
    pub event_type: EventTypeName,
    /// Simulation payload. Pass a JSON object that matches the schema for an event type to simulate a custom payload. If omitted, Paddle populates with a demo example.
    pub payload: serde_json::Value,
    /// Information about the request. Sent by Paddle as part of the simulation.
    pub request: SimulationEventRequest,
    /// Information about the response. Sent by the responding server for the notification setting.
    pub response: SimulationEventResponse,
    /// RFC 3339 datetime string of when this entity was created. Set automatically by Paddle.
    pub created_at: String,
    /// RFC 3339 datetime string of when this entity was updated. Set automatically by Paddle.
    pub updated_at: String,
}

/// Represents a simulation run entity for a scenario.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationRunScenario {
    /// Unique Paddle ID for this simulation run, prefixed with `ntfsimrun_`.
    pub id: SimulationRunID,
    /// Status of this simulation run.
    pub status: SimulationRunStatus,
    /// RFC 3339 datetime string of when this entity was created. Set automatically by Paddle.
    pub created_at: String,
    /// RFC 3339 datetime string of when this entity was updated. Set automatically by Paddle.
    pub updated_at: String,
    /// Scenario for a simulation.
    pub r#type: SimulationScenarioType,
}

/// Represents a simulation run entity for a single event.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationRunSingleEvent {
    /// Unique Paddle ID for this simulation run, prefixed with `ntfsimrun_`.
    pub id: SimulationRunID,
    /// Status of this simulation run.
    pub status: SimulationRunStatus,
    /// RFC 3339 datetime string of when this entity was created. Set automatically by Paddle.
    pub created_at: String,
    /// RFC 3339 datetime string of when this entity was updated. Set automatically by Paddle.
    pub updated_at: String,
    /// Type of event sent by Paddle, in the format `entity.event_type`.
    pub r#type: EventTypeName,
}

/// Represents a simulation entity for a scenario.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationScenario {
    /// Unique Paddle ID for this simulation, prefixed with `ntfsim_`.
    pub id: SimulationID,
    /// Whether this entity can be used in Paddle.
    pub status: Status,
    /// Unique Paddle ID for this notification setting, prefixed with `ntfset_`.
    pub notification_setting_id: NotificationSettingID,
    /// Name of this simulation.
    pub name: String,
    /// Scenario for a simulation.
    pub r#type: SimulationScenarioType,
    /// Simulation payload. `null` for scenarios.
    pub payload: Option<serde_json::Value>,
    /// RFC 3339 datetime string of when this simulation was last run. `null` until run. Set automatically by Paddle.
    pub last_run_at: Option<DateTime<FixedOffset>>,
    /// RFC 3339 datetime string of when this entity was created. Set automatically by Paddle.
    pub created_at: String,
    /// RFC 3339 datetime string of when this entity was updated. Set automatically by Paddle.
    pub updated_at: String,
}

/// Represents a simulation entity for a scenario when creating.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationScenarioCreate {
    /// Unique Paddle ID for this notification setting, prefixed with `ntfset_`.
    pub notification_setting_id: NotificationSettingID,
    /// Name of this simulation.
    pub name: String,
    /// Scenario for a simulation.
    pub r#type: SimulationScenarioType,
}

/// Represents a simulation entity for a scenario when updating.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationScenarioUpdate {
    /// Unique Paddle ID for this notification setting, prefixed with `ntfset_`.
    pub notification_setting_id: NotificationSettingID,
    /// Name of this simulation.
    pub name: Option<String>,
    /// Whether this entity can be used in Paddle.
    pub status: Status,
    /// Scenario for a simulation.
    pub r#type: SimulationScenarioType,
}

/// Represents a simulation entity for a single event when creating.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationSingleEventCreate {
    /// Unique Paddle ID for this notification setting, prefixed with `ntfset_`.
    pub notification_setting_id: NotificationSettingID,
    /// Name of this simulation.
    pub name: Option<String>,
    /// Type of event sent by Paddle, in the format `entity.event_type`.
    pub r#type: EventTypeName,
    /// Simulation payload. Pass a JSON object that matches the schema for an event type to simulate a custom payload. If omitted, Paddle populates with a demo example.
    pub payload: Option<serde_json::Value>,
}

/// Represents a simulation entity for a single event when updating.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationSingleEventUpdate {
    /// Unique Paddle ID for this notification setting, prefixed with `ntfset_`.
    pub notification_setting_id: NotificationSettingID,
    /// Name of this simulation.
    pub name: Option<String>,
    /// Whether this entity can be used in Paddle.
    pub status: Status,
    /// Type of event sent by Paddle, in the format `entity.event_type`.
    pub r#type: EventTypeName,
    /// Simulation payload. Pass a JSON object that matches the schema for an event type to simulate a custom payload. Set to `null` to clear and populate with a demo example.
    pub payload: Option<serde_json::Value>,
}

/// Represents a simulation type.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationType {
    /// Type of simulation sent by Paddle. Single event simulations are in the format `entity.event_type`; scenario simulations are in `snake_case`.
    pub name: String,
    /// Descriptive label for this simulation type. Typically gives more context about a scenario. Single event simulations are in the format `entity.event_type`.
    pub label: String,
    /// Short description of this simulation type.
    pub description: String,
    /// Group for this simulation type. Typically the entity that this event relates to.
    pub group: String,
    /// Type of simulation.
    pub r#type: SimulationKind,
    /// List of events that will be sent for this simulation type.
    pub events: Vec<EventTypeName>,
}

/// Change that's scheduled to be applied to a subscription. Use the pause subscription, cancel subscription, and resume subscription operations to create scheduled changes. `null` if no scheduled changes.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionScheduledChange {
    /// Kind of change that's scheduled to be applied to this subscription.
    pub action: ScheduledChangeAction,
    /// RFC 3339 datetime string.
    pub effective_at: DateTime<FixedOffset>,
    /// RFC 3339 datetime string of when a paused subscription should resume. Only used for `pause` scheduled changes.
    pub resume_at: Option<DateTime<FixedOffset>>,
}

/// Authenticated customer portal deep links for this subscription. For security, the `token` appended to each link is temporary. You shouldn't store these links.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionManagementUrls {
    /// Link to the page for this subscription in the customer portal with the payment method update form pre-opened. Use as part of workflows to let customers update their payment details. `null` for manually-collected subscriptions.
    pub update_payment_method: Option<String>,
    /// Link to the page for this subscription in the customer portal with the subscription cancellation form pre-opened. Use as part of cancel subscription workflows.
    pub cancel: String,
}

/// Represents a subscription entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Subscription {
    /// Unique Paddle ID for this subscription entity, prefixed with `sub_`.
    pub id: SubscriptionID,
    /// Status of this subscription. Set automatically by Paddle. Use the pause subscription or cancel subscription operations to change.
    pub status: SubscriptionStatus,
    /// Unique Paddle ID for this customer entity, prefixed with `ctm_`.
    pub customer_id: CustomerID,
    /// Unique Paddle ID for this address entity, prefixed with `add_`.
    pub address_id: AddressID,
    /// Paddle ID of the business that this subscription is for, prefixed with `biz_`.
    pub business_id: Option<BusinessID>,
    /// Supported three-letter ISO 4217 currency code.
    pub currency_code: CurrencyCode,
    /// RFC 3339 datetime string of when this entity was created. Set automatically by Paddle.
    pub created_at: DateTime<Utc>,
    /// RFC 3339 datetime string of when this entity was updated. Set automatically by Paddle.
    pub updated_at: DateTime<Utc>,
    /// RFC 3339 datetime string of when this subscription started. This may be different from `first_billed_at` if the subscription started in trial.
    pub started_at: Option<DateTime<Utc>>,
    /// RFC 3339 datetime string of when this subscription was first billed. This may be different from `started_at` if the subscription started in trial.
    pub first_billed_at: Option<DateTime<Utc>>,
    /// RFC 3339 datetime string of when this subscription is next scheduled to be billed.
    pub next_billed_at: Option<DateTime<Utc>>,
    /// RFC 3339 datetime string of when this subscription was paused. Set automatically by Paddle when the pause subscription operation is used. `null` if not paused.
    pub paused_at: Option<DateTime<Utc>>,
    /// RFC 3339 datetime string of when this subscription was canceled. Set automatically by Paddle when the cancel subscription operation is used. `null` if not canceled.
    pub canceled_at: Option<DateTime<Utc>>,
    /// Details of the discount applied to this subscription.
    pub discount: Option<SubscriptionDiscountTimePeriod>,
    /// How payment is collected. `automatic` for checkout, `manual` for invoices.
    pub collection_mode: CollectionMode,
    /// Details for invoicing. Required if `collection_mode` is `manual`.
    pub billing_details: Option<BillingDetails>,
    /// Current billing period for this subscription. Set automatically by Paddle based on the billing cycle. `null` for `paused` and `canceled` subscriptions.
    pub current_billing_period: Option<TimePeriod>,
    pub billing_cycle: Duration,
    /// Change that's scheduled to be applied to a subscription. Use the pause subscription, cancel subscription, and resume subscription operations to create scheduled changes. `null` if no scheduled changes.
    pub scheduled_change: Option<SubscriptionScheduledChange>,
    /// Authenticated customer portal deep links for this subscription. For security, the `token` appended to each link is temporary. You shouldn't store these links.
    pub management_urls: Option<SubscriptionManagementUrls>,
    /// List of items on this subscription. Only recurring items are returned.
    pub items: Vec<SubscriptionItem>,
    /// Your own structured key-value data.
    pub custom_data: Option<serde_json::Value>,
    /// Import information for this entity. `null` if this entity is not imported.
    pub import_meta: Option<ImportMeta>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SubscriptionChargeItem {
    /// Add a catalog item to a subscription. In this case, the product and price that you're billing for exist in your product catalog in Paddle.
    CatalogItem(SubscriptionItemCreateWithPriceId),
    /// Add a non-catalog price for an existing product in your catalog to a subscription. In this case, the product you're billing for is a catalog product, but you charge a specific price for it.
    NonCatalogExistingProduct(SubscriptionChargeCreateWithPrice),
    /// Add a non-catalog price for a non-catalog product in your catalog to a subscription. In this case, the product and price that you're billing for are specific to this subscription.
    NonCatalogPriceAndProduct(SubscriptionChargeCreateWithPriceAndProduct),
}

/// Represents a one-time charge for a subscription.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionCharge {
    /// When this subscription change should take effect from. Defaults to `next_billing_period`, which creates a
    /// `scheduled_change` to apply the subscription change at the end of the billing period.
    pub effective_from: EffectiveFrom,
    /// List of one-time charges to bill for. Only prices where the `billing_cycle` is `null` may be added.
    ///
    /// You can charge for items that you've added to your catalog by passing the Paddle ID of an existing price entity, or you can charge for non-catalog items by passing a price object.
    ///
    /// Non-catalog items can be for existing products, or you can pass a product object as part of your price to charge for a non-catalog product.
    pub items: Vec<SubscriptionChargeItem>,
    /// How Paddle should handle changes made to a subscription or its items if the payment fails during update. If omitted, defaults to `prevent_change`.
    pub on_payment_failure: SubscriptionOnPaymentFailure,
}

/// Breakdown of a charge in the lowest denomination of a currency (e.g. cents for USD).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Totals {
    /// Subtotal before discount, tax, and deductions. If an item, unit price multiplied by quantity.
    pub subtotal: String,
    /// Total discount as a result of any discounts applied.
    ///
    /// Except for percentage discounts, Paddle applies tax to discounts based on the line item `price.tax_mode`. If `price.tax_mode` for a line item is `internal`, Paddle removes tax from the discount applied.
    pub discount: String,
    /// Total tax on the subtotal.
    pub tax: String,
    /// Total after discount and tax.
    pub total: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaxRatesUsed {
    /// Rate used to calculate tax for this transaction preview.
    pub tax_rate: String,
    /// Breakdown of a charge in the lowest denomination of a currency (e.g. cents for USD).
    pub totals: Totals,
}

/// Breakdown of the total for a transaction. These numbers can be negative when dealing with subscription updates that result in credit.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionTotals {
    /// Subtotal before discount, tax, and deductions. If an item, unit price multiplied by quantity.
    pub subtotal: String,
    /// Total discount as a result of any discounts applied.
    ///
    /// Except for percentage discounts, Paddle applies tax to discounts based on the line item `price.tax_mode`. If `price.tax_mode` for a line item is `internal`, Paddle removes tax from the discount applied.
    pub discount: String,
    /// Total tax on the subtotal.
    pub tax: String,
    /// Total after discount and tax.
    pub total: String,
    /// Total credit applied to this transaction. This includes credits applied using a customer's credit balance and adjustments to a `billed` transaction.
    pub credit: String,
    /// Additional credit generated from negative `details.line_items`. This credit is added to the customer balance.
    pub credit_to_balance: String,
    /// Total due on a transaction after credits and any payments.
    pub balance: String,
    /// Total due on a transaction after credits but before any payments.
    pub grand_total: String,
    /// Total fee taken by Paddle for this transaction. `null` until the transaction is `completed` and the fee is processed.
    pub fee: Option<String>,
    /// Total earnings for this transaction. This is the total minus the Paddle fee. `null` until the transaction is `completed` and the fee is processed.
    pub earnings: Option<String>,
    /// Three-letter ISO 4217 currency code of the currency used for this transaction.
    pub currency_code: CurrencyCode,
}

/// SubscriptionTransactionDetailsPreview requires same fields as TransactionLineItemPreview but proration is optional
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionTransactionDetailsPreviewItem {
    /// Paddle ID for the price related to this transaction line item, prefixed with `pri_`.
    /// The value is null for custom prices being previewed.
    pub price_id: Option<PriceID>,
    /// Quantity of this transaction line item.
    pub quantity: i64,
    /// Rate used to calculate tax for this transaction line item.
    pub tax_rate: String,
    /// Breakdown of a charge in the lowest denomination of a currency (e.g. cents for USD).
    pub unit_totals: Totals,
    /// Breakdown of a charge in the lowest denomination of a currency (e.g. cents for USD).
    pub totals: Totals,
    /// Represents a product (preview) entity.
    pub product: ProductPreview,
    /// How proration was calculated for this item.
    pub proration: Option<Proration>,
}

/// Calculated totals for a transaction preview, including discounts, tax, and currency conversion. Considered the source of truth for totals on a transaction preview.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionTransactionDetailsPreview {
    /// List of tax rates applied to this transaction preview.
    pub tax_rates_used: Vec<TaxRatesUsed>,
    /// Breakdown of the total for a transaction. These numbers can be negative when dealing with subscription updates that result in credit.
    pub totals: TransactionTotals,
    /// Information about line items for this transaction preview. Different from transaction preview `items` as they include totals calculated by Paddle. Considered the source of truth for line item totals.
    pub line_items: Vec<SubscriptionTransactionDetailsPreviewItem>,
}

/// Preview of the next transaction for this subscription. May include prorated charges that aren't yet billed and one-time charges. `null` if the subscription is scheduled to cancel or pause.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NextTransaction {
    pub billing_period: TimePeriod,
    /// Calculated totals for a transaction preview, including discounts, tax, and currency conversion. Considered the source of truth for totals on a transaction preview.
    pub details: SubscriptionTransactionDetailsPreview,
    /// Preview of adjustments for the next transaction.
    pub adjustments: Vec<AdjustmentPreview>,
}

/// Represents a subscription entity with related entities included.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionWithInclude {
    /// The subscription entity.
    #[serde(flatten)]
    pub subscription: Subscription,
    /// Preview of the next transaction for this subscription. May include prorated charges that aren't yet billed and one-time charges. Returned when the `include` parameter is used with the `next_transaction` value. `null` if the subscription is scheduled to cancel or pause.
    pub next_transaction: Option<NextTransaction>,
    /// Preview of the recurring transaction for this subscription. This is what the customer can expect to be billed when there are no prorated or one-time charges. Returned when the `include` parameter is used with the `recurring_transaction_details` value.
    pub recurring_transaction_details: Option<SubscriptionTransactionDetailsPreview>,
}

/// Details of the result of credits and charges. Where the total of any credit adjustments is greater than the total charge, the result is a prorated credit; otherwise, the result is a prorated charge.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateSummaryResult {
    /// Whether the subscription change results in a prorated credit or a charge.
    pub action: UpdateSummaryResultAction,
    /// Amount representing the result of this update, either a charge or a credit.
    pub amount: String,
    /// Supported three-letter ISO 4217 currency code.
    pub currency_code: CurrencyCode,
}

/// Impact of this subscription change. Includes whether the change results in a charge or credit, and totals for prorated amounts.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionPreviewUpdateSummary {
    /// A base representation of monetary value unformatted in the lowest denomination with currency code.
    pub credit: Money,
    /// A base representation of monetary value unformatted in the lowest denomination with currency code.
    pub charge: Money,
    /// Details of the result of credits and charges. Where the total of any credit adjustments is greater than the total charge, the result is a prorated credit; otherwise, the result is a prorated charge.
    pub result: UpdateSummaryResult,
}

/// Represents a subscription preview when previewing a subscription.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionPreview {
    /// Status of this subscription. Set automatically by Paddle. Use the pause subscription or cancel subscription operations to change.
    pub status: SubscriptionStatus,
    /// Unique Paddle ID for this customer entity, prefixed with `ctm_`.
    pub customer_id: CustomerID,
    /// Unique Paddle ID for this address entity, prefixed with `add_`.
    pub address_id: AddressID,
    /// Paddle ID of the business that this subscription is for, prefixed with `biz_`.
    pub business_id: Option<BusinessID>,
    /// Supported three-letter ISO 4217 currency code.
    pub currency_code: CurrencyCode,
    /// RFC 3339 datetime string of when this entity was created. Set automatically by Paddle.
    pub created_at: DateTime<Utc>,
    /// RFC 3339 datetime string of when this entity was updated. Set automatically by Paddle.
    pub updated_at: DateTime<Utc>,
    /// RFC 3339 datetime string of when this subscription started. This may be different from `first_billed_at` if the subscription started in trial.
    pub started_at: Option<DateTime<Utc>>,
    /// RFC 3339 datetime string of when this subscription was first billed. This may be different from `started_at` if the subscription started in trial.
    pub first_billed_at: Option<DateTime<Utc>>,
    /// RFC 3339 datetime string of when this subscription is next scheduled to be billed.
    pub next_billed_at: Option<DateTime<Utc>>,
    /// RFC 3339 datetime string of when this subscription was paused. Set automatically by Paddle when the pause subscription operation is used. `null` if not paused.
    pub paused_at: Option<DateTime<Utc>>,
    /// RFC 3339 datetime string of when this subscription was canceled. Set automatically by Paddle when the cancel subscription operation is used. `null` if not canceled.
    pub canceled_at: Option<DateTime<Utc>>,
    /// Details of the discount applied to this subscription.
    pub discount: Option<Discount>,
    /// How payment is collected. `automatic` for checkout, `manual` for invoices.
    pub collection_mode: CollectionMode,
    /// Details for invoicing. Required if `collection_mode` is `manual`.
    pub billing_details: Option<BillingDetails>,
    /// Current billing period for this subscription. Set automatically by Paddle based on the billing cycle. `null` for `paused` and `canceled` subscriptions.
    pub current_billing_period: Option<TimePeriod>,
    pub billing_cycle: Duration,
    /// Change that's scheduled to be applied to a subscription. Use the pause subscription, cancel subscription, and resume subscription operations to create scheduled changes. `null` if no scheduled changes.
    pub scheduled_change: Option<SubscriptionScheduledChange>,
    /// Authenticated customer portal deep links for this subscription. For security, the `token` appended to each link is temporary. You shouldn't store these links.
    pub management_urls: SubscriptionManagementUrls,
    /// List of items on this subscription. Only recurring items are returned.
    pub items: Vec<SubscriptionItem>,
    /// Your own structured key-value data.
    pub custom_data: Option<serde_json::Value>,
    /// Preview of the immediate transaction created as a result of changes to the subscription. Returns a complete object where `proration_billing_mode` is `prorated_immediately` or `full_immediately`; `null` otherwise.
    pub immediate_transaction: Option<NextTransaction>,
    /// Preview of the next transaction for this subscription. Includes charges created where `proration_billing_mode` is `prorated_next_billing_period` or `full_next_billing_period`, as well as one-time charges. `null` if the subscription is scheduled to cancel or pause.
    pub next_transaction: NextTransaction,
    /// Calculated totals for a transaction preview, including discounts, tax, and currency conversion. Considered the source of truth for totals on a transaction preview.
    pub recurring_transaction_details: SubscriptionTransactionDetailsPreview,
    pub update_summary: Option<SubscriptionPreviewUpdateSummary>,
    /// Import information for this entity. `null` if this entity is not imported.
    pub import_meta: Option<ImportMeta>,
}

/// Details of the discount applied to this subscription. Include to add a discount to a subscription. `null` to remove a discount.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionDiscountEffectiveFrom {
    /// Unique Paddle ID for this discount, prefixed with `dsc_`.
    pub id: DiscountID,
    /// When this subscription change should take effect from. Defaults to `next_billing_period`, which creates a
    /// `scheduled_change` to apply the subscription change at the end of the billing period.
    pub effective_from: EffectiveFrom,
}

/// Represents a subscription entity when updating subscriptions.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionUpdate {
    /// Unique Paddle ID for this customer entity, prefixed with `ctm_`.
    pub customer_id: CustomerID,
    /// Unique Paddle ID for this address entity, prefixed with `add_`.
    pub address_id: AddressID,
    /// Paddle ID of the business that this subscription is for, prefixed with `biz_`. Include to change the business for a subscription.
    pub business_id: BusinessID,
    /// Supported three-letter ISO 4217 currency code.
    pub currency_code: CurrencyCode,
    /// RFC 3339 datetime string.
    pub next_billed_at: DateTime<Utc>,
    /// Details of the discount applied to this subscription. Include to add a discount to a subscription. `null` to remove a discount.
    pub discount: SubscriptionDiscountEffectiveFrom,
    /// How payment is collected. `automatic` for checkout, `manual` for invoices.
    pub collection_mode: CollectionMode,
    /// Details for invoicing. Required if `collection_mode` is `manual`. `null` if changing `collection_mode` to `automatic`.
    pub billing_details: BillingDetails,
    /// Change that's scheduled to be applied to a subscription. When updating, you may only set to `null` to remove a scheduled change. Use the pause subscription, cancel subscription, and resume subscription operations to create scheduled changes.
    pub scheduled_change: Option<()>,
    /// List of items on this subscription. Only recurring items may be added. Send the complete list of items that should be on this subscription, including existing items to retain.
    pub items: Vec<SubscriptionChargeItem>,
    /// Your own structured key-value data.
    pub custom_data: Option<serde_json::Value>,
    /// How Paddle should handle proration calculation for changes made to a subscription or its items. Required when making
    /// changes that impact billing.
    ///
    /// For automatically-collected subscriptions, responses may take longer than usual if a proration billing mode that
    /// collects for payment immediately is used.
    pub proration_billing_mode: ProrationBillingMode,
    /// How Paddle should handle changes made to a subscription or its items if the payment fails during update. If omitted, defaults to `prevent_change`.
    pub on_payment_failure: SubscriptionOnPaymentFailure,
}

/// Price object for a non-catalog item to bill for. Include a `product_id` to relate this non-catalog price to an existing catalog price.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionChargeCreateWithPricePrice {
    /// Unique Paddle ID for this product, prefixed with `pro_`.
    pub product_id: ProductID,
    /// Internal description for this price, not shown to customers. Typically notes for your team.
    pub description: String,
    /// Name of this price, shown to customers at checkout and on invoices. Typically describes how often the related product bills.
    pub name: Option<String>,
    /// How tax is calculated for this price.
    pub tax_mode: TaxMode,
    /// A base representation of monetary value unformatted in the lowest denomination with currency code.
    pub unit_price: Money,
    /// List of unit price overrides. Use to override the base price with a custom price and currency for a country or group of countries.
    pub unit_price_overrides: Vec<UnitPriceOverride>,
    pub quantity: PriceQuantity,
    /// Your own structured key-value data.
    pub custom_data: Option<serde_json::Value>,
}

/// Price object for a non-catalog item to charge for. Include a `product` object to create a non-catalog product for this non-catalog price.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionChargeCreateWithPriceAndProduct {
    /// Internal description for this price, not shown to customers. Typically notes for your team.
    pub description: String,
    /// Name of this price, shown to customers at checkout and on invoices. Typically describes how often the related product bills.
    pub name: Option<String>,
    /// How tax is calculated for this price.
    pub tax_mode: TaxMode,
    /// A base representation of monetary value unformatted in the lowest denomination with currency code.
    pub unit_price: Money,
    /// List of unit price overrides. Use to override the base price with a custom price and currency for a country or group of countries.
    pub unit_price_overrides: Vec<UnitPriceOverride>,
    pub quantity: PriceQuantity,
    /// Your own structured key-value data.
    pub custom_data: Option<serde_json::Value>,
    /// Product object for a non-catalog item to charge for.
    pub product: TransactionSubscriptionProductCreate,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionChargeCreateWithPrice {
    /// Quantity to bill for.
    pub quantity: i64,
    /// Price object for a non-catalog item to bill for. Include a `product_id` to relate this non-catalog price to an existing catalog price.
    pub price: SubscriptionChargeCreateWithPricePrice,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionPriceCreateWithProductId {
    /// Internal description for this price, not shown to customers. Typically notes for your team.
    pub description: String,
    /// Name of this price, shown to customers at checkout and on invoices. Typically describes how often the related product bills.
    pub name: Option<String>,
    /// How often this price should be charged. `null` if price is non-recurring (one-time).
    pub billing_cycle: Option<Duration>,
    /// Trial period for the product related to this price. The billing cycle begins once the trial period is over. `null` for no trial period. Requires `billing_cycle`.
    pub trial_period: Option<Duration>,
    /// How tax is calculated for this price.
    pub tax_mode: TaxMode,
    /// A base representation of monetary value unformatted in the lowest denomination with currency code.
    pub unit_price: Money,
    /// List of unit price overrides. Use to override the base price with a custom price and currency for a country or group of countries.
    pub unit_price_overrides: Vec<UnitPriceOverride>,
    pub quantity: PriceQuantity,
    /// Your own structured key-value data.
    pub custom_data: Option<serde_json::Value>,
    /// Paddle ID for the product that this price is for, prefixed with `pro_`.
    pub product_id: ProductID,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionItemCreateWithPrice {
    /// Quantity to bill for.
    pub quantity: i64,
    pub price: TransactionPriceCreateWithProductId,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionItemCreateWithPriceId {
    /// Quantity to bill for.
    pub quantity: i64,
    /// Unique Paddle ID for this price, prefixed with `pri_`.
    pub price_id: PriceID,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionUpdateItem {
    /// Unique Paddle ID for this price, prefixed with `pri_`.
    pub price_id: PriceID,
    /// Quantity of this item to add to the subscription. If updating an existing item and not changing the quantity, you may omit `quantity`.
    pub quantity: i64,
}

/// Breakdown of a charge in the lowest denomination of a currency (e.g. cents for USD).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TotalsWithoutDiscount {
    /// Subtotal before tax, and deductions. If an item, unit price multiplied by quantity.
    pub subtotal: String,
    /// Total tax on the subtotal.
    pub tax: String,
    /// Total after tax.
    pub total: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionItem {
    /// Represents a price entity.
    pub price: Price,
    /// Quantity of this item on the transaction.
    pub quantity: i64,
    /// How proration was calculated for this item. Populated when a transaction is created from a subscription change, where `proration_billing_mode` was `prorated_immediately` or `prorated_next_billing_period`. Set automatically by Paddle.
    pub proration: Option<Proration>,
}

/// Breakdown of the totals for a transaction after adjustments.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionTotalsAdjusted {
    /// Subtotal before discount, tax, and deductions. If an item, unit price multiplied by quantity.
    pub subtotal: String,
    /// Total tax on the subtotal.
    pub tax: String,
    /// Total after tax.
    pub total: String,
    /// Total due after credits but before any payments.
    pub grand_total: String,
    /// Total fee taken by Paddle for this transaction. `null` until the transaction is `completed` and the fee is processed.
    pub fee: Option<String>,
    /// Total earnings for this transaction. This is the total minus the Paddle fee.
    /// `null` until the transaction is `completed` and the fee is processed.
    pub earnings: Option<String>,
    /// Supported three-letter ISO 4217 currency code.
    pub currency_code: CurrencyCode,
}

/// Breakdown of the payout total for a transaction after adjustments. `null` until the transaction is `completed`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionPayoutTotalsAdjusted {
    /// Total before tax and fees.
    pub subtotal: String,
    /// Total tax on the subtotal.
    pub tax: String,
    /// Total after tax.
    pub total: String,
    /// Total fee taken by Paddle for this payout.
    pub fee: String,
    /// Details of any chargeback fees incurred for this transaction.
    pub chargeback_fee: ChargebackFee,
    /// Total earnings for this payout. This is the subtotal minus the Paddle fee, excluding chargeback fees.
    pub earnings: String,
    /// Supported three-letter ISO 4217 currency code for payouts from Paddle.
    pub currency_code: CurrencyCodePayouts,
}

/// Information about line items for this transaction. Different from transaction `items` as they include totals calculated by Paddle. Considered the source of truth for line item totals.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionLineItemWithId {
    /// Unique Paddle ID for this transaction item, prefixed with `txnitm_`.
    pub id: TransactionItemID,
    /// Unique Paddle ID for this price, prefixed with `pri_`.
    pub price_id: PriceID,
    /// Quantity of this transaction line item.
    pub quantity: i64,
    /// How proration was calculated for this item. Populated when a transaction is created from a subscription change, where `proration_billing_mode` was `prorated_immediately` or `prorated_next_billing_period`. Set automatically by Paddle.
    pub proration: Option<Proration>,
    /// Rate used to calculate tax for this transaction line item.
    pub tax_rate: String,
    /// Breakdown of a charge in the lowest denomination of a currency (e.g. cents for USD).
    pub unit_totals: Totals,
    /// Breakdown of a charge in the lowest denomination of a currency (e.g. cents for USD).
    pub totals: Totals,
    /// Represents a product entity.
    pub product: Product,
}

/// Calculated totals for a transaction, including proration, discounts, tax, and currency conversion. Considered the source of truth for totals on a transaction.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionDetails {
    /// List of tax rates applied for this transaction.
    pub tax_rates_used: Vec<TaxRatesUsed>,
    /// Breakdown of the total for a transaction. These numbers can be negative when dealing with subscription updates that result in credit.
    pub totals: TransactionTotals,
    /// Breakdown of the totals for a transaction after adjustments.
    pub adjusted_totals: TransactionTotalsAdjusted,
    /// Breakdown of the payout total for a transaction. `null` until the transaction is `completed`. Returned in your payout currency.
    pub payout_totals: Option<TransactionPayoutTotals>,
    /// Breakdown of the payout total for a transaction after adjustments. `null` until the transaction is `completed`.
    pub adjusted_payout_totals: Option<TransactionPayoutTotalsAdjusted>,
    /// Information about line items for this transaction. Different from transaction `items` as they include totals calculated by Paddle. Considered the source of truth for line item totals.
    pub line_items: Vec<TransactionLineItemWithId>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionPaymentAttempt {
    /// UUID for this payment attempt.
    pub payment_attempt_id: String,
    /// UUID for the stored payment method used for this payment attempt. Deprecated - use `payment_method_id` instead.
    pub stored_payment_method_id: String,
    /// Paddle ID of the payment method used for this payment attempt, prefixed with `paymtd_`.
    pub payment_method_id: Option<PaymentMethodID>,
    /// Amount for collection in the lowest denomination of a currency (e.g. cents for USD).
    pub amount: String,
    /// Status of this payment attempt.
    pub status: PaymentAttemptStatus,
    /// Reason why a payment attempt failed. Returns `null` if payment captured successfully.
    pub error_code: Option<ErrorCode>,
    /// Information about the payment method used for a payment attempt.
    pub method_details: MethodDetails,
    /// RFC 3339 datetime string of when this entity was created. Set automatically by Paddle.
    pub created_at: String,
    /// RFC 3339 datetime string of when this payment was captured. `null` if `status` is not `captured`.
    pub captured_at: Option<DateTime<FixedOffset>>,
}

/// Paddle Checkout details for this transaction. Returned for automatically-collected transactions and where `billing_details.enable_checkout` is `true` for manually-collected transactions; `null` otherwise.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionCheckout {
    /// Paddle Checkout URL for this transaction, composed of the URL passed in the request or your default payment URL + `?_ptxn=` and the Paddle ID for this transaction.
    pub url: Option<String>,
}

/// Contains an invoice PDF url for a transaction.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionInvoice {
    /// URL of the requested resource.
    pub url: Option<String>,
}

/// Represents a transaction entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    /// Unique Paddle ID for this transaction entity, prefixed with `txn_`.
    pub id: TransactionID,
    /// Status of this transaction. You may set a transaction to `billed` or `canceled`, other statuses are set automatically by Paddle. Automatically-collected transactions may return `completed` if payment is captured successfully, or `past_due` if payment failed.
    pub status: TransactionStatus,
    /// Paddle ID of the customer that this transaction is for, prefixed with `ctm_`.
    pub customer_id: Option<CustomerID>,
    /// Paddle ID of the address that this transaction is for, prefixed with `add_`.
    pub address_id: Option<AddressID>,
    /// Paddle ID of the business that this transaction is for, prefixed with `biz_`.
    pub business_id: Option<BusinessID>,
    /// Your own structured key-value data.
    //pub custom_data: Option<serde_json::Value>,
    pub custom_data: Option<serde_json::Value>,
    /// Supported three-letter ISO 4217 currency code.
    pub currency_code: CurrencyCode,
    /// Describes how this transaction was created.
    pub origin: TransactionOrigin,
    /// Paddle ID of the subscription that this transaction is for, prefixed with `sub_`.
    pub subscription_id: Option<SubscriptionID>,
    /// Paddle ID of the invoice that this transaction is related to, prefixed with `inv_`. Used for compatibility with the Paddle Invoice API, which is now deprecated. This field is scheduled to be removed in the next version of the Paddle API.
    pub invoice_id: Option<InvoiceId>,
    /// Invoice number for this transaction. Automatically generated by Paddle when you mark a transaction as `billed` where `collection_mode` is `manual`.
    pub invoice_number: Option<String>,
    /// How payment is collected. `automatic` for checkout, `manual` for invoices.
    pub collection_mode: CollectionMode,
    /// Paddle ID of the discount applied to this transaction, prefixed with `dsc_`.
    pub discount_id: Option<DiscountID>,
    /// Details for invoicing. Required if `collection_mode` is `manual`.
    pub billing_details: Option<BillingDetails>,
    /// Time period that this transaction is for. Set automatically by Paddle for subscription renewals to describe the period that charges are for.
    pub billing_period: Option<TimePeriod>,
    /// List of items on this transaction. For calculated totals, use `details.line_items`.
    pub items: Vec<TransactionItem>,
    /// Calculated totals for a transaction, including proration, discounts, tax, and currency conversion. Considered the source of truth for totals on a transaction.
    pub details: TransactionDetails,
    /// List of payment attempts for this transaction, including successful payments. Sorted by `created_at` in descending order, so most recent attempts are returned first.
    pub payments: Vec<TransactionPaymentAttempt>,
    /// Paddle Checkout details for this transaction. Returned for automatically-collected transactions and where `billing_details.enable_checkout` is `true` for manually-collected transactions; `null` otherwise.
    pub checkout: TransactionCheckout,
    /// RFC 3339 datetime string of when this entity was created. Set automatically by Paddle.
    pub created_at: DateTime<Utc>,
    /// RFC 3339 datetime string of when this entity was updated. Set automatically by Paddle.
    pub updated_at: DateTime<Utc>,
    /// RFC 3339 datetime string of when this transaction was marked as `billed`. `null` for transactions that aren't `billed` or `completed`. Set automatically by Paddle.
    pub billed_at: Option<DateTime<Utc>>,
    /// RFC 3339 datetime string of when a transaction was revised. Revisions describe an update to customer information for a billed or completed transaction. `null` if not revised. Set automatically by Paddle.
    pub revised_at: Option<DateTime<Utc>>,
}

/// Represents a transaction entity when creating transactions.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionCreate {
    /// Unique Paddle ID for this transaction entity, prefixed with `txn_`.
    pub id: TransactionID,
    /// Status of this transaction. You may set a transaction to `billed` or `canceled`, other statuses are set automatically by Paddle. Automatically-collected transactions may return `completed` if payment is captured successfully, or `past_due` if payment failed.
    pub status: TransactionStatus,
    /// Paddle ID of the customer that this transaction is for, prefixed with `ctm_`. If omitted, transaction status is `draft`.
    pub customer_id: CustomerID,
    /// Paddle ID of the address that this transaction is for, prefixed with `add_`. Requires `customer_id`. If omitted, transaction status is `draft`.
    pub address_id: AddressID,
    /// Paddle ID of the business that this transaction is for, prefixed with `biz_`. Requires `customer_id`.
    pub business_id: BusinessID,
    /// Your own structured key-value data.
    pub custom_data: Option<serde_json::Value>,
    /// Supported three-letter ISO 4217 currency code. Must be `USD`, `EUR`, or `GBP` if `collection_mode` is `manual`.
    pub currency_code: CurrencyCode,
    /// Describes how this transaction was created.
    pub origin: TransactionOrigin,
    /// Paddle ID of the subscription that this transaction is for, prefixed with `sub_`.
    pub subscription_id: SubscriptionID,
    /// Paddle ID of the invoice that this transaction is related to, prefixed with `inv_`. Used for compatibility with the Paddle Invoice API, which is now deprecated. This field is scheduled to be removed in the next version of the Paddle API.
    pub invoice_id: InvoiceId,
    /// Invoice number for this transaction. Automatically generated by Paddle when you mark a transaction as `billed` where `collection_mode` is `manual`.
    pub invoice_number: Option<String>,
    /// How payment is collected. `automatic` for checkout, `manual` for invoices.
    pub collection_mode: CollectionMode,
    /// Paddle ID of the discount applied to this transaction, prefixed with `dsc_`.
    pub discount_id: DiscountID,
    /// Details for invoicing. Required if `collection_mode` is `manual`.
    pub billing_details: BillingDetails,
    /// Time period that this transaction is for. Set automatically by Paddle for subscription renewals to describe the period that charges are for.
    pub billing_period: Option<TimePeriod>,
    /// List of items to charge for. You can charge for items that you've added to your catalog by passing the Paddle ID of an existing price entity, or you can charge for non-catalog items by passing a price object.
    ///
    /// Non-catalog items can be for existing products, or you can pass a product object as part of your price to charge for a non-catalog product.
    pub items: Vec<SubscriptionChargeItem>,
    /// Calculated totals for a transaction, including proration, discounts, tax, and currency conversion. Considered the source of truth for totals on a transaction.
    pub details: TransactionDetails,
    /// List of payment attempts for this transaction, including successful payments. Sorted by `created_at` in descending order, so most recent attempts are returned first.
    pub payments: Vec<TransactionPaymentAttempt>,
    /// Paddle Checkout details for this transaction. You may pass a URL when creating or updating an automatically-collected transaction, or when creating or updating a manually-collected transaction where `billing_details.enable_checkout` is `true`.
    pub checkout: TransactionCheckout,
    /// RFC 3339 datetime string of when this entity was created. Set automatically by Paddle.
    pub created_at: DateTime<FixedOffset>,
    /// RFC 3339 datetime string of when this entity was updated. Set automatically by Paddle.
    pub updated_at: DateTime<FixedOffset>,
    /// RFC 3339 datetime string of when this transaction was marked as `billed`. `null` for transactions that aren't `billed` or `completed`. Set automatically by Paddle.
    pub billed_at: Option<DateTime<FixedOffset>>,
}

/// Information about line items for this transaction preview. Different from transaction preview `items` as they include totals calculated by Paddle. Considered the source of truth for line item totals.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionLineItemPreview {
    /// Paddle ID for the price related to this transaction line item, prefixed with `pri_`.
    /// The value is null for custom prices being previewed.
    pub price_id: Option<PriceID>,
    /// Quantity of this transaction line item.
    pub quantity: i64,
    /// Rate used to calculate tax for this transaction line item.
    pub tax_rate: String,
    /// Breakdown of a charge in the lowest denomination of a currency (e.g. cents for USD).
    pub unit_totals: Totals,
    /// Breakdown of a charge in the lowest denomination of a currency (e.g. cents for USD).
    pub totals: Totals,
    /// Represents a product (preview) entity.
    pub product: ProductPreview,
    /// How proration was calculated for this item.
    pub proration: Option<Proration>,
}

/// Calculated totals for a transaction preview, including discounts, tax, and currency conversion. Considered the source of truth for totals on a transaction preview.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionDetailsPreview {
    /// List of tax rates applied to this transaction preview.
    pub tax_rates_used: Vec<TaxRatesUsed>,
    /// Breakdown of the total for a transaction. These numbers can be negative when dealing with subscription updates that result in credit.
    pub totals: TransactionTotals,
    /// Information about line items for this transaction preview. Different from transaction preview `items` as they include totals calculated by Paddle. Considered the source of truth for line item totals.
    pub line_items: Vec<TransactionLineItemPreview>,
}

/// Represents a transaction entity when previewing transactions.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionPreview {
    /// Paddle ID of the customer that this transaction preview is for, prefixed with `ctm_`.
    pub customer_id: Option<CustomerID>,
    /// Paddle ID of the address that this transaction preview is for, prefixed with `add_`. Send one of `address_id`, `customer_ip_address`, or the `address` object when previewing.
    pub address_id: Option<AddressID>,
    /// Paddle ID of the business that this transaction preview is for, prefixed with `biz_`.
    pub business_id: Option<BusinessID>,
    /// Supported three-letter ISO 4217 currency code.
    pub currency_code: CurrencyCode,
    /// Paddle ID of the discount applied to this transaction preview, prefixed with `dsc_`.
    pub discount_id: Option<DiscountID>,
    /// IP address for this transaction preview. Send one of `address_id`, `customer_ip_address`, or the `address` object when previewing.
    pub customer_ip_address: Option<String>,
    /// Address for this transaction preview. Send one of `address_id`, `customer_ip_address`, or the `address` object when previewing.
    pub address: Option<AddressPreview>,
    /// Whether trials should be ignored for transaction preview calculations.
    ///
    /// By default, recurring items with trials are considered to have a zero charge when previewing. Set to `true` to disable this.
    pub ignore_trials: bool,
    /// List of items to preview transaction calculations for.
    pub items: Vec<TransactionItemPreviewBase>,
    /// Calculated totals for a transaction preview, including discounts, tax, and currency conversion. Considered the source of truth for totals on a transaction preview.
    pub details: TransactionDetailsPreview,
    pub available_payment_methods: Vec<PaymentMethodType>,
}

/// Represents an entity for previewing prices.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PricingPreview {
    /// Paddle ID of the customer that this transaction preview is for, prefixed with `ctm_`.
    pub customer_id: Option<CustomerID>,
    /// Paddle ID of the address that this transaction preview is for, prefixed with `add_`. Send one of `address_id`, `customer_ip_address`, or the `address` object when previewing.
    pub address_id: Option<AddressID>,
    /// Paddle ID of the business that this transaction preview is for, prefixed with `biz_`.
    pub business_id: Option<BusinessID>,
    /// Supported three-letter ISO 4217 currency code.
    pub currency_code: CurrencyCode,
    /// Paddle ID of the discount applied to this transaction preview, prefixed with `dsc_`.
    pub discount_id: Option<DiscountID>,
    /// Address for this transaction preview. Send one of `address_id`, `customer_ip_address`, or the `address` object when previewing.
    pub address: Option<AddressPreview>,
    /// IP address for this transaction preview. Send one of `address_id`, `customer_ip_address`, or the `address` object when previewing.
    pub customer_ip_address: Option<String>,
    /// Calculated totals for a transaction preview, including discounts, tax, and currency conversion. Considered the source of truth for totals on a transaction preview.
    pub details: PricePreviewDetails,
    pub available_payment_methods: Vec<PaymentMethodType>,
}

/// Represents a transaction entity when previewing.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionPreviewCreate {
    /// Paddle ID of the customer that this transaction preview is for, prefixed with `ctm_`.
    pub customer_id: CustomerID,
    /// Supported three-letter ISO 4217 currency code.
    pub currency_code: CurrencyCode,
    /// Paddle ID of the discount applied to this transaction preview, prefixed with `dsc_`.
    pub discount_id: DiscountID,
    /// Whether trials should be ignored for transaction preview calculations.
    ///
    /// By default, recurring items with trials are considered to have a zero charge when previewing. Set to `true` to disable this.
    pub ignore_trials: bool,
    /// List of items to preview charging for. You can preview charging for items that you've added to your catalog by passing the Paddle ID of an existing price entity, or you can preview charging for non-catalog items by passing a price object.
    ///
    /// Non-catalog items can be for existing products, or you can pass a product object as part of your price to preview charging for a non-catalog product.
    pub items: Vec<SubscriptionChargeItem>,
}

/// Represents a price entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionPriceCreateBase {
    /// Internal description for this price, not shown to customers. Typically notes for your team.
    pub description: String,
    /// Name of this price, shown to customers at checkout and on invoices. Typically describes how often the related product bills.
    pub name: Option<String>,
    /// How often this price should be charged. `null` if price is non-recurring (one-time).
    pub billing_cycle: Option<Duration>,
    /// Trial period for the product related to this price. The billing cycle begins once the trial period is over. `null` for no trial period. Requires `billing_cycle`.
    pub trial_period: Option<Duration>,
    /// How tax is calculated for this price.
    pub tax_mode: TaxMode,
    /// A base representation of monetary value unformatted in the lowest denomination with currency code.
    pub unit_price: Money,
    /// List of unit price overrides. Use to override the base price with a custom price and currency for a country or group of countries.
    pub unit_price_overrides: Vec<UnitPriceOverride>,
    pub quantity: PriceQuantity,
    /// Your own structured key-value data.
    pub custom_data: Option<serde_json::Value>,
}

/// Represents a customer information revision for a transaction.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionRevise {
    /// Revised customer information for this transaction.
    pub customer: Customer,
    /// Revised business information for this transaction.
    pub business: Business,
    /// Revised address information for this transaction.
    pub address: Address,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionSubscriptionProductCreate {
    /// Name of this product.
    pub name: String,
    /// Short description for this product.
    pub description: Option<String>,
    /// Tax category for this product. Used for charging the correct rate of tax. Selected tax category must be enabled on your Paddle account.
    pub tax_category: TaxCategory,
    /// Image for this product. Included in the checkout and on some customer documents.
    pub image_url: Option<String>,
    /// Your own structured key-value data.
    pub custom_data: Option<serde_json::Value>,
}

/// Represents a transaction entity when updating transactions.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionUpdate {
    /// Unique Paddle ID for this transaction entity, prefixed with `txn_`.
    pub id: TransactionID,
    /// Status of this transaction. You may set a transaction to `billed` or `canceled`, other statuses are set automatically by Paddle. Automatically-collected transactions may return `completed` if payment is captured successfully, or `past_due` if payment failed.
    pub status: TransactionStatus,
    /// Paddle ID of the customer that this transaction is for, prefixed with `ctm_`.
    pub customer_id: CustomerID,
    /// Paddle ID of the address that this transaction is for, prefixed with `add_`.
    pub address_id: AddressID,
    /// Paddle ID of the business that this transaction is for, prefixed with `biz_`.
    pub business_id: BusinessID,
    /// Your own structured key-value data.
    pub custom_data: Option<serde_json::Value>,
    /// Supported three-letter ISO 4217 currency code. Must be `USD`, `EUR`, or `GBP` if `collection_mode` is `manual`.
    pub currency_code: CurrencyCode,
    /// Describes how this transaction was created.
    pub origin: TransactionOrigin,
    /// Paddle ID of the subscription that this transaction is for, prefixed with `sub_`.
    pub subscription_id: SubscriptionID,
    /// Paddle ID of the invoice that this transaction is related to, prefixed with `inv_`. Used for compatibility with the Paddle Invoice API, which is now deprecated. This field is scheduled to be removed in the next version of the Paddle API.
    pub invoice_id: InvoiceId,
    /// Invoice number for this transaction. Automatically generated by Paddle when you mark a transaction as `billed` where `collection_mode` is `manual`.
    pub invoice_number: Option<String>,
    /// How payment is collected. `automatic` for checkout, `manual` for invoices.
    pub collection_mode: CollectionMode,
    /// Paddle ID of the discount applied to this transaction, prefixed with `dsc_`.
    pub discount_id: DiscountID,
    /// Details for invoicing. Required if `collection_mode` is `manual`.
    pub billing_details: BillingDetails,
    /// Time period that this transaction is for. Set automatically by Paddle for subscription renewals to describe the period that charges are for.
    pub billing_period: Option<TimePeriod>,
    /// List of items on this transaction.
    ///
    /// When making a request, each object must contain either a `price_id` or a `price` object, and a `quantity`.
    ///
    /// Include a `price_id` to charge for an existing catalog item, or a `price` object to charge for a non-catalog item.
    pub items: Vec<SubscriptionChargeItem>,
    /// Calculated totals for a transaction, including proration, discounts, tax, and currency conversion. Considered the source of truth for totals on a transaction.
    pub details: TransactionDetails,
    /// List of payment attempts for this transaction, including successful payments. Sorted by `created_at` in descending order, so most recent attempts are returned first.
    pub payments: Vec<TransactionPaymentAttempt>,
    /// Paddle Checkout details for this transaction. You may pass a URL when creating or updating an automatically-collected transaction, or when creating or updating a manually-collected transaction where `billing_details.enable_checkout` is `true`.
    pub checkout: TransactionCheckout,
    /// RFC 3339 datetime string of when this entity was created. Set automatically by Paddle.
    pub created_at: DateTime<FixedOffset>,
    /// RFC 3339 datetime string of when this entity was updated. Set automatically by Paddle.
    pub updated_at: DateTime<FixedOffset>,
    /// RFC 3339 datetime string of when this transaction was marked as `billed`. `null` for transactions that aren't `billed` or `completed`. Set automatically by Paddle.
    pub billed_at: Option<DateTime<FixedOffset>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionItemCreateBase {
    /// Quantity of this item on the transaction.
    pub quantity: i64,
    /// How proration was calculated for this item. Populated when a transaction is created from a subscription change, where `proration_billing_mode` was `prorated_immediately` or `prorated_next_billing_period`. Set automatically by Paddle.
    pub proration: Proration,
}

/// Information about line items for this transaction. Different from transaction `items` as they include totals calculated by Paddle. Considered the source of truth for line item totals.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionLineItem {
    /// Unique Paddle ID for this price, prefixed with `pri_`.
    pub price_id: PriceID,
    /// Quantity of this transaction line item.
    pub quantity: i64,
    /// How proration was calculated for this item. Populated when a transaction is created from a subscription change, where `proration_billing_mode` was `prorated_immediately` or `prorated_next_billing_period`. Set automatically by Paddle.
    pub proration: Proration,
    /// Rate used to calculate tax for this transaction line item.
    pub tax_rate: String,
    /// Breakdown of a charge in the lowest denomination of a currency (e.g. cents for USD).
    pub unit_totals: Totals,
    /// Breakdown of a charge in the lowest denomination of a currency (e.g. cents for USD).
    pub totals: Totals,
    /// Represents a product entity.
    pub product: Product,
}

/// Breakdown of the payout total for a transaction. `null` until the transaction is `completed`. Returned in your payout currency.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionPayoutTotals {
    /// Total before tax and fees.
    pub subtotal: String,
    /// Total discount as a result of any discounts applied.
    /// Except for percentage discounts, Paddle applies tax to discounts based on the line item `price.tax_mode`. If `price.tax_mode` for a line item is `internal`, Paddle removes tax from the discount applied.
    pub discount: String,
    /// Total tax on the subtotal.
    pub tax: String,
    /// Total after tax.
    pub total: String,
    /// Total credit applied to this transaction. This includes credits applied using a customer's credit balance and adjustments to a `billed` transaction.
    pub credit: String,
    /// Additional credit generated from negative `details.line_items`. This credit is added to the customer balance.
    pub credit_to_balance: String,
    /// Total due on a transaction after credits and any payments.
    pub balance: String,
    /// Total due on a transaction after credits but before any payments.
    pub grand_total: String,
    /// Total fee taken by Paddle for this payout.
    pub fee: String,
    /// Total earnings for this payout. This is the subtotal minus the Paddle fee.
    pub earnings: String,
    /// Supported three-letter ISO 4217 currency code for payouts from Paddle.
    pub currency_code: CurrencyCodePayouts,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionItemPreviewBase {
    /// Quantity of this item on the transaction.
    pub quantity: i64,
    /// Whether this item should be included in totals for this transaction preview. Typically used to exclude one-time charges from calculations.
    pub include_in_totals: bool,
    /// How proration was calculated for this item. `null` for transaction previews.
    pub proration: Option<Proration>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PricePreviewBase {
    /// Paddle ID of the customer that this preview is for, prefixed with `ctm_`.
    pub customer_id: CustomerID,
    /// Paddle ID of the address that this preview is for, prefixed with `add_`. Send one of `address_id`, `customer_ip_address`, or the `address` object when previewing.
    pub address_id: AddressID,
    /// Paddle ID of the business that this preview is for, prefixed with `biz_`.
    pub business_id: BusinessID,
    /// Supported three-letter ISO 4217 currency code.
    pub currency_code: CurrencyCode,
    /// Paddle ID of the discount applied to this preview, prefixed with `dsc_`.
    pub discount_id: DiscountID,
    /// Address for this preview. Send one of `address_id`, `customer_ip_address`, or the `address` object when previewing.
    pub address: Address,
    /// IP address for this transaction preview. Send one of `address_id`, `customer_ip_address`, or the `address` object when previewing.
    pub customer_ip_address: Option<String>,
}

/// Array of discounts applied to this preview line item. Empty if no discounts applied.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PricePreviewDiscounts {
    /// Represents a discount entity.
    pub discount: Discount,
    /// Total amount discounted as a result of this discount.
    pub total: String,
    /// Total amount discounted as a result of this discount in the format of a given currency. '
    pub formatted_total: String,
}

/// Information about line items for this preview. Includes totals calculated by Paddle. Considered the source of truth for line item totals.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PricePreviewLineItem {
    /// Represents a price entity.
    pub price: Price,
    /// Quantity of this preview line item.
    pub quantity: i64,
    /// Rate used to calculate tax for this preview line item.
    pub tax_rate: String,
    /// Breakdown of a charge in the lowest denomination of a currency (e.g. cents for USD).
    pub unit_totals: Totals,
    /// Breakdown of a charge in the lowest denomination of a currency (e.g. cents for USD).
    pub formatted_unit_totals: Totals,
    /// Breakdown of a charge in the lowest denomination of a currency (e.g. cents for USD).
    pub totals: Totals,
    /// Breakdown of a charge in the lowest denomination of a currency (e.g. cents for USD).
    pub formatted_totals: Totals,
    /// Represents a product entity.
    pub product: Product,
    pub discounts: Vec<PricePreviewDiscounts>,
}

/// Payout entity received from a payout event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Payout {
    /// ID for this payout.
    pub id: PayoutID,
    /// Status of this payout.
    pub status: PayoutStatus,
    /// Fee amount for this chargeback in the original currency.
    pub amount: String,
    /// Three-letter ISO 4217 currency code for chargeback fees.
    pub currency_code: CurrencyCodeChargebacks,
}

/// ApiKey entity
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiKey {
    /// Unique Paddle ID for this API key entity, prefixed with apikey_.
    pub id: ApiKeyID,
    /// Short name of this API key. Typically unique and human-identifiable.
    pub name: String,
    /// Short description of this API key. Typically gives details about what the API key is used for and where it's used.
    pub description: Option<String>,
    /// An obfuscated version of this API key, prefixed with `pdl_` and containing `_apikey_`.
    pub key: String,
    /// Status of this API key.
    pub status: ApiKeyStatus,
    /// Permissions assigned to this API key. Determines what actions the API key can perform.
    pub permissions: Vec<String>,
    /// Datetime of when this API key expires.
    pub expires_at: Option<DateTime<Utc>>,
    /// Datetime of when this API key was last used (accurate to within 1 hour). null if never used.
    pub last_used_at: Option<DateTime<Utc>>,
    /// Datetime of when this entity was created. Set automatically by Paddle.
    pub created_at: DateTime<Utc>,
    /// Datetime of when this entity was updated. Set automatically by Paddle.
    pub updated_at: DateTime<Utc>,
}

/// Calculated totals for a price preview, including discounts, tax, and currency conversion.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PricePreviewDetails {
    pub line_items: Vec<PricePreviewLineItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PricePreviewItem {
    /// Unique Paddle ID for this price, prefixed with `pri_`.
    pub price_id: PriceID,
    /// Quantity of the item to preview.
    pub quantity: i64,
}

/// Price object for a non-catalog item to charge for.
///
/// Can be for existing products, or you can pass a product object as part of your price to charge for a non-catalog product.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct TransactionItemNonCatalogPrice {
    description: String,
    name: Option<String>,
    billing_cycle: Option<Duration>,
    trial_period: Option<Duration>,
    tax_mode: Option<TaxMode>,
    unit_price: Money,
    unit_price_overrides: Option<Vec<UnitPriceOverride>>,
    quantity: Option<PriceQuantity>,
    custom_data: Option<serde_json::Value>,
    product_id: Option<ProductID>,
    product: Option<TransactionSubscriptionProductCreate>,
}

impl TransactionItemNonCatalogPrice {
    /// Create new price object for non-catalog item.
    ///
    /// - `description` - Internal description for this price, not shown to customers. Typically notes for your team.
    /// - `amount` - Amount in the lowest denomination for the currency, e.g. 10 USD = 1000 (cents). Although represented as a string, this value must be a valid integer.
    /// - `currency` - Currency code.
    pub fn new(description: impl Into<String>, amount: u64, currency: CurrencyCode) -> Self {
        Self {
            description: description.into(),
            name: None,
            billing_cycle: None,
            trial_period: None,
            tax_mode: None,
            unit_price: Money {
                amount: amount.to_string(),
                currency_code: currency,
            },
            unit_price_overrides: None,
            quantity: None,
            custom_data: None,
            product_id: None,
            product: None,
        }
    }

    /// Name of this price, shown to customers at checkout and on invoices. Typically describes how often the related product bills.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// How often this price should be charged.
    pub fn billing_cycle(mut self, billing_cycle: Duration) -> Self {
        self.billing_cycle = Some(billing_cycle);
        self
    }

    /// Trial period for the product related to this price. The billing cycle begins once the trial period is over.
    pub fn trial_period(mut self, trial_period: Duration) -> Self {
        self.trial_period = Some(trial_period);
        self
    }

    /// How tax is calculated for this price.
    pub fn tax_mode(mut self, tax_mode: TaxMode) -> Self {
        self.tax_mode = Some(tax_mode);
        self
    }

    /// Use to override the base price with a custom price and currency for a country or group of countries.
    /// See [UnitPriceOverride] for more information.
    /// See [CountryCodeSupported] for more information.
    /// See [Money] for more information.
    /// See [CurrencyCode] for more information.
    pub fn add_unit_price_override(
        mut self,
        country_codes: impl IntoIterator<Item = CountryCodeSupported>,
        amount: u64,
        currency: CurrencyCode,
    ) -> Self {
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
    pub fn set_unit_price_overrides(mut self, overrides: Vec<UnitPriceOverride>) -> Self {
        self.unit_price_overrides = Some(overrides);
        self
    }

    /// Limits on how many times the related product can be purchased at this price. Useful for discount campaigns. If omitted, defaults to 1-100.
    pub fn quantity(mut self, quantity: PriceQuantity) -> Self {
        self.quantity = Some(quantity);
        self
    }

    /// Your own structured key-value data.
    pub fn custom_data(mut self, custom_data: serde_json::Value) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Paddle ID of the product that this price is for, prefixed with `prd_`.
    pub fn product_id(mut self, product_id: impl Into<ProductID>) -> Self {
        self.product_id = Some(product_id.into());
        self.product = None;
        self
    }

    /// Product object for a non-catalog item to charge for.
    ///
    /// Setting a non-catalog product to this price will override the catalog product id.
    pub fn product(mut self, product: TransactionSubscriptionProductCreate) -> Self {
        self.product = Some(product);
        self.product_id = None;
        self
    }
}
