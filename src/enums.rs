//! This module contains enums used in the Paddle API.

#![allow(clippy::upper_case_acronyms, clippy::enum_variant_names)]

use serde::{Deserialize, Serialize};
#[cfg(feature = "strum")]
use strum::{Display, EnumString};

use crate::reports::ReportType;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[non_exhaustive]
pub enum CountryCodeSupported {
    /// Andorra
    AD,
    /// United Arab Emirates
    AE,
    /// Antigua and Barbuda
    AG,
    /// Anguilla
    AI,
    /// Albania
    AL,
    /// Armenia
    AM,
    /// Angola
    AO,
    /// Argentina
    AR,
    /// American Samoa
    AS,
    /// Austria
    AT,
    /// Australia
    AU,
    /// Aruba
    AW,
    /// Åland Islands
    AX,
    /// Azerbaijan
    AZ,
    /// Bosnia and Herzegovina
    BA,
    /// Barbados
    BB,
    /// Bangladesh
    BD,
    /// Belgium
    BE,
    /// Burkina Faso
    BF,
    /// Bulgaria
    BG,
    /// Bahrain
    BH,
    /// Burundi
    BI,
    /// Benin
    BJ,
    /// Saint Barthélemy
    BL,
    /// Bermuda
    BM,
    /// Brunei
    BN,
    /// Bolivia
    BO,
    /// Caribbean Netherlands (Bonaire, Sint Eustatius, and Saba)
    BQ,
    /// Brazil
    BR,
    /// Bahamas
    BS,
    /// Bhutan
    BT,
    /// Bouvet Island
    BV,
    /// Botswana
    BW,
    /// Belize
    BZ,
    /// Canada
    CA,
    /// Cocos Islands
    CC,
    /// Republic of Congo
    CG,
    /// Switzerland
    CH,
    /// Côte d'Ivoire (Ivory Coast)
    CI,
    /// Cook Islands
    CK,
    /// Chile
    CL,
    /// Cameroon
    CM,
    /// China
    CN,
    /// Colombia
    CO,
    /// Costa Rica
    CR,
    /// Cape Verde
    CV,
    /// Curaçao
    CW,
    /// Christmas Island
    CX,
    /// Cyprus
    CY,
    /// Czechia (Czech Republic)
    CZ,
    /// Germany
    DE,
    /// Djibouti
    DJ,
    /// Denmark
    DK,
    /// Dominica
    DM,
    /// Dominican Republic
    DO,
    /// Algeria
    DZ,
    /// Ecuador
    EC,
    /// Estonia
    EE,
    /// Egypt
    EG,
    /// Western Sahara
    EH,
    /// Eritrea
    ER,
    /// Spain
    ES,
    /// Ethiopia
    ET,
    /// Finland
    FI,
    /// Fiji
    FJ,
    /// Falkland Islands
    FK,
    /// Micronesia
    FM,
    /// Faroe Islands
    FO,
    /// France
    FR,
    /// Gabon
    GA,
    /// United Kingdom
    GB,
    /// Grenada
    GD,
    /// Georgia
    GE,
    /// French Guiana
    GF,
    /// Guernsey
    GG,
    /// Ghana
    GH,
    /// Gibraltar
    GI,
    /// Greenland
    GL,
    /// Gambia
    GM,
    /// Guinea
    GN,
    /// Guadeloupe
    GP,
    /// Equatorial Guinea
    GQ,
    /// Greece
    GR,
    /// South Georgia and the South Sandwich Islands
    GS,
    /// Guatemala
    GT,
    /// Guam
    GU,
    /// Guinea-Bissau
    GW,
    /// Guyana
    GY,
    /// Hong Kong
    HK,
    /// Heard Island and McDonald Islands
    HM,
    /// Honduras
    HN,
    /// Croatia
    HR,
    /// Hungary
    HU,
    /// Indonesia
    ID,
    /// Ireland
    IE,
    /// Israel
    IL,
    /// Isle of Man
    IM,
    /// India
    IN,
    /// British Indian Ocean Territory
    IO,
    /// Iraq
    IQ,
    /// Iceland
    IS,
    /// Italy
    IT,
    /// Jersey
    JE,
    /// Jamaica
    JM,
    /// Jordan
    JO,
    /// Japan
    JP,
    /// Kenya
    KE,
    /// Kyrgyzstan
    KG,
    /// Cambodia
    KH,
    /// Kiribati
    KI,
    /// Comoros
    KM,
    /// Saint Kitts and Nevis
    KN,
    /// South Korea
    KR,
    /// Kuwait
    KW,
    /// Cayman Islands
    KY,
    /// Kazakhstan
    KZ,
    /// Lao People's Democratic Republic (Laos)
    LA,
    /// Lebanon
    LB,
    /// Saint Lucia
    LC,
    /// Liechtenstein
    LI,
    /// Sri Lanka
    LK,
    /// Liberia
    LR,
    /// Lesotho
    LS,
    /// Lithuania
    LT,
    /// Luxembourg
    LU,
    /// Latvia
    LV,
    /// Morocco
    MA,
    /// Monaco
    MC,
    /// Moldova
    MD,
    /// Montenegro
    ME,
    /// Saint Martin
    MF,
    /// Madagascar
    MG,
    /// Marshall Islands
    MH,
    /// Macedonia
    MK,
    /// Mongolia
    MN,
    /// Macao
    MO,
    /// Northern Mariana Islands
    MP,
    /// Martinique
    MQ,
    /// Mauritania
    MR,
    /// Montserrat
    MS,
    /// Malta
    MT,
    /// Mauritius
    MU,
    /// Maldives
    MV,
    /// Malawi
    MW,
    /// Mexico
    MX,
    /// Malaysia
    MY,
    /// Mozambique
    MZ,
    /// Namibia
    NA,
    /// New Caledonia
    NC,
    /// Niger
    NE,
    /// Norfolk Island
    NF,
    /// Nigeria
    NG,
    /// Netherlands
    NL,
    /// Norway
    NO,
    /// Nepal
    NP,
    /// Nauru
    NR,
    /// Niue
    NU,
    /// New Zealand
    NZ,
    /// Oman
    OM,
    /// Panama
    PA,
    /// Peru
    PE,
    /// French Polynesia
    PF,
    /// Papua New Guinea
    PG,
    /// Philippines
    PH,
    /// Pakistan
    PK,
    /// Poland
    PL,
    /// Saint Pierre and Miquelon
    PM,
    /// Pitcairn
    PN,
    /// Puerto Rico
    PR,
    /// Palestinian territories
    PS,
    /// Portugal
    PT,
    /// Palau
    PW,
    /// Paraguay
    PY,
    /// Qatar
    QA,
    /// Reunion
    RE,
    /// Romania
    RO,
    /// Republic of Serbia
    RS,
    /// Rwanda
    RW,
    /// Saudi Arabia
    SA,
    /// Solomon Islands
    SB,
    /// Seychelles
    SC,
    /// Sweden
    SE,
    /// Singapore
    SG,
    /// Saint Helena
    SH,
    /// Slovenia
    SI,
    /// Svalbard and Jan Mayen
    SJ,
    /// Slovakia
    SK,
    /// Sierra Leone
    SL,
    /// San Marino
    SM,
    /// Senegal
    SN,
    /// Suriname
    SR,
    /// São Tomé and Príncipe
    ST,
    /// El Salvador
    SV,
    /// Sint Maarten
    SX,
    /// Swaziland
    SZ,
    /// Turks and Caicos Islands
    TC,
    /// Chad
    TD,
    /// French Southern and Antarctic Lands
    TF,
    /// Togo
    TG,
    /// Thailand
    TH,
    /// Tajikistan
    TJ,
    /// Tokelau
    TK,
    /// Timor-Leste
    TL,
    /// Turkmenistan
    TM,
    /// Tunisia
    TN,
    /// Tonga
    TO,
    /// Turkey
    TR,
    /// Trinidad and Tobago
    TT,
    /// Tuvalu
    TV,
    /// Taiwan
    TW,
    /// Tanzania
    TZ,
    /// Ukraine
    UA,
    /// Uganda
    UG,
    /// United States Minor Outlying Islands
    UM,
    /// United States
    US,
    /// Uruguay
    UY,
    /// Uzbekistan
    UZ,
    /// Holy See (Vatican City)
    VA,
    /// Saint Vincent and the Grenadines
    VC,
    /// British Virgin Islands
    VG,
    /// U.S. Virgin Islands
    VI,
    /// Vietnam
    VN,
    /// Vanuatu
    VU,
    /// Wallis and Futuna
    WF,
    /// Samoa
    WS,
    /// Kosovo
    XK,
    /// Mayotte
    YT,
    /// South Africa
    ZA,
    /// Zambia
    ZM,
    /// Other country code
    #[serde(untagged)]
    #[cfg_attr(feature = "strum", strum(default))]
    Other(String),
}

/// Whether this entity can be used in Paddle.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "kebab-case"))]
#[non_exhaustive]
pub enum Status {
    /// Entity is active and can be used.
    Active,
    /// Entity is archived, so can't be used.
    Archived,
}

/// How this adjustment impacts the related transaction.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum AdjustmentAction {
    /// Credits some or all the related transaction.
    Credit,
    /// Refunds some or all the related transaction. Must be approved by Paddle in most cases.
    Refund,
    /// Chargeback for the related transaction. Automatically created by Paddle when a customer successfully disputes a charge.
    Chargeback,
    /// Reversal of a chargeback for the related transaction. Automatically created by Paddle when Paddle contests a chargeback successfully.
    ChargebackReverse,
    /// Warning of an upcoming chargeback for the related transaction. Automatically created by Paddle.
    ChargebackWarning,
    /// Reversal of a credit for the related transaction. Automatically created by Paddle.
    CreditReverse,
}

/// Type of adjustment. Use `full` to adjust the grand total for the related transaction. Include an `items` array when creating a `partial` adjustment. If omitted, defaults to `partial`.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "strum", strum(serialize_all = "lowercase"))]
#[non_exhaustive]
pub enum AdjustmentType {
    /// The grand total for the related transaction is adjusted.
    Full,
    /// Some line items for the related transaction are adjusted. Requires `items`.
    Partial,
}

/// Supported three-letter ISO 4217 currency code.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[non_exhaustive]
pub enum CurrencyCode {
    /// United States Dollar
    USD,
    /// Euro
    EUR,
    /// Pound Sterling
    GBP,
    /// Japanese Yen
    JPY,
    /// Australian Dollar
    AUD,
    /// Canadian Dollar
    CAD,
    /// Swiss Franc
    CHF,
    /// Hong Kong Dollar
    HKD,
    /// Singapore Dollar
    SGD,
    /// Swedish Krona
    SEK,
    /// Argentine Peso
    ARS,
    /// Brazilian Real
    BRL,
    /// Chinese Yuan
    CNY,
    /// Colombian Peso
    COP,
    /// Czech Koruna
    CZK,
    /// Danish Krone
    DKK,
    /// Hungarian Forint
    HUF,
    /// Israeli Shekel
    ILS,
    /// Indian Rupee
    INR,
    /// South Korean Won
    KRW,
    /// Mexican Peso
    MXN,
    /// Norwegian Krone
    NOK,
    /// New Zealand Dollar
    NZD,
    /// Polish Zloty
    PLN,
    /// Russian Ruble
    RUB,
    /// Thai Baht
    THB,
    /// Turkish Lira
    TRY,
    /// New Taiwan Dollar
    TWD,
    /// Ukrainian Hryvnia
    UAH,
    /// Vietnamese Dong
    VND,
    /// South African Rand
    ZAR,
}

/// Status of this adjustment. Set automatically by Paddle.
///
/// Most refunds for live accounts are created with the status of `pending_approval` until reviewed by Paddle, but some are automatically approved. For sandbox accounts, Paddle automatically approves refunds every ten minutes.
///
/// Credit adjustments don't require approval from Paddle, so they're created as `approved`.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum AdjustmentStatus {
    /// Adjustment is pending approval by Paddle. Most refunds for live accounts must be approved by Paddle.
    PendingApproval,
    /// Adjustment is approved. Default for credits. Set when Paddle approves a refund that was `pending_approval`.
    Approved,
    /// Adjustment has been rejected. Set when Paddle rejects a refund that was `pending_approval`.
    Rejected,
    /// Adjustment has been reversed. Set by Paddle when a `chargeback_reversal` or `credit_reversal` adjustment is created for this adjustment.
    Reversed,
}

/// Three-letter ISO 4217 currency code for chargeback fees.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[non_exhaustive]
pub enum CurrencyCodeChargebacks {
    /// Australian Dollar
    AUD,
    /// Canadian Dollar
    CAD,
    /// Euro
    EUR,
    /// Pound Sterling
    GBP,
    /// United States Dollar
    USD,
}

/// Supported three-letter ISO 4217 currency code for payouts from Paddle.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[non_exhaustive]
pub enum CurrencyCodePayouts {
    /// Australian Dollar
    AUD,
    /// Canadian Dollar
    CAD,
    /// Swiss Franc
    CHF,
    /// Chinese Yuan
    CNY,
    /// Czech Koruna
    CZK,
    /// Danish Krone
    DKK,
    /// Euro
    EUR,
    /// Pound Sterling
    GBP,
    /// Hungarian Forint
    HUF,
    /// Polish Zloty
    PLN,
    /// Swedish Krona
    SEK,
    /// United States Dollar
    USD,
    /// South African Rand
    ZAR,
}

/// Type of adjustment for this transaction item. `tax` adjustments are automatically created by Paddle.
/// Include `amount` when creating a `partial` adjustment.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "strum", strum(serialize_all = "lowercase"))]
#[non_exhaustive]
pub enum AdjustmentItemType {
    /// Full total for this transaction item is adjusted.
    Full,
    /// Part of this transaction item is adjusted. Include `amount` to specify the partial amount adjusted.
    Partial,
    /// Tax for this transaction item is adjusted. Created automatically by Paddle.
    Tax,
    /// A prorated amount for this transaction item is adjusted. Created automatically by Paddle in some cases when making changes to a subscription.
    Proration,
}

/// Unit of time.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "kebab-case"))]
#[non_exhaustive]
pub enum Interval {
    Day,
    Week,
    Month,
    Year,
}

/// Type of credit or debit card used to pay.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum CardType {
    /// American Express
    AmericanExpress,
    /// Diners Club
    DinersClub,
    /// Discover Card
    Discover,
    /// JCB Card, popular in Japan
    Jcb,
    /// Mada Card, popular in Saudi Arabia
    Mada,
    /// Maestro (debit card)
    Maestro,
    /// Mastercard
    Mastercard,
    /// UnionPay, popular in China
    UnionPay,
    /// Card type unknown
    Unknown,
    /// Visa
    Visa,
}

/// Type of item. Standard items are considered part of your catalog and are shown on the Paddle dashboard.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "kebab-case"))]
#[non_exhaustive]
pub enum CatalogType {
    /// Non-catalog item. Typically created for a specific transaction or subscription. Not returned when listing or shown in the Paddle dashboard.
    Custom,
    /// Standard item. Can be considered part of your catalog and reused across transactions and subscriptions easily.
    Standard,
}

/// How payment is collected. `automatic` for checkout, `manual` for invoices.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum CollectionMode {
    /// Payment is collected automatically using a checkout initially, then using a payment method on file.
    Automatic,
    /// Payment is collected manually. Customers are sent an invoice with payment terms and can make a payment offline or using a checkout. Requires `billing_details`.
    Manual,
}

/// Type of payment method saved.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum SavedPaymentMethodType {
    /// Alipay, popular in China.
    Alipay,
    /// Apple Pay on a supported Apple device.
    ApplePay,
    /// Credit or debit card.
    Card,
    /// Google Pay on a supported Android device, Chromebook, or Google Chrome browser.
    GooglePay,
    /// PayPal.
    Paypal,
}

/// Describes how this payment method was saved.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum PaymentMethodOrigin {
    /// The customer chose to save this payment method while purchasing a one-time item.
    SavedDuringPurchase,
    /// The customer purchased a subscription, so this payment method was saved for future purchases.
    Subscription,
}

/// Whether this entity can be used in Paddle.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "kebab-case"))]
#[non_exhaustive]
pub enum DiscountStatus {
    /// Entity is active and can be used.
    Active,
    /// Entity is archived, so can't be used.
    Archived,
}

/// Type of discount. Determines how this discount impacts the checkout or transaction total.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum DiscountType {
    /// Discounts a checkout or transaction by a flat amount, for example -$100. Requires `currency_code`.
    Flat,
    /// Discounts a checkout or transaction by a flat amount per unit, for example -$100 per user. Requires `currency_code`.
    FlatPerSeat,
    /// Discounts a checkout or transaction by a percentage of the total, for example -10%. Maximum 100%.
    Percentage,
}

/// When this subscription change should take effect from. Defaults to `next_billing_period`, which creates a
/// `scheduled_change` to apply the subscription change at the end of the billing period.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum EffectiveFrom {
    /// Takes effect on the next billing period.
    NextBillingPeriod,
    /// Takes effect immediately.
    Immediately,
}

/// Type of error encountered.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[non_exhaustive]
pub enum Type {
    /// Typically means there's a problem with the request that you made.
    RequestError,
    /// Typically means there's a problem with the Paddle API.
    ApiError,
}

/// Reason why a payment attempt failed. Returns `null` if payment captured successfully.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum ErrorCode {
    /// Cancellation not possible because the amount has already been canceled. Not typically returned for payments.
    AlreadyCanceled,
    /// Refund is not possible because the amount has already been refunded. Not typically returned for payments.
    AlreadyRefunded,
    /// Payment required a 3DS2 authentication challenge. The customer completed the challenge but was not successful.
    AuthenticationFailed,
    /// Payment method issuer has indicated that the card cannot be used as it is frozen, lost, damaged, or stolen.
    BlockedCard,
    /// Customer has requested that the mandate for recurring payments be canceled.
    Canceled,
    /// Payment method has been declined, with no other information returned.
    Declined,
    /// Payment method has been declined, and the issuer has indicated that it should not be retried. This could mean the account is closed or the customer revoked authorization to charge the payment method.
    DeclinedNotRetryable,
    /// Payment method issuer has indicated that this card is expired. Expired cards may also return `invalid_payment_details`, depending on how a payment is routed.
    ExpiredCard,
    /// Payment method issuer or payment service provider flagged this payment as potentially fraudulent.
    Fraud,
    /// Payment method issuer or payment service provider cannot process a payment that is this high or low.
    InvalidAmount,
    /// Payment service provider has indicated the payment method isn't valid. This typically means that it's expired. `expired_card` is returned by the payment method issuer, rather than the payment service provider.
    InvalidPaymentDetails,
    /// Payment service provider couldn't reach the payment method issuer.
    IssuerUnavailable,
    /// Payment method declined because of insufficient funds, or fund limits being reached.
    NotEnoughBalance,
    /// Payment method has been declined because the network scheme that the customer selected isn't supported by the payment service provider.
    PreferredNetworkNotSupported,
    /// Something went wrong with the payment service provider, with no other information returned.
    PspError,
    /// Payment service provider didn't receive payment method information as they've been redacted.
    RedactedPaymentMethod,
    /// Something went wrong with the Paddle platform. Try again later, or check status.paddle.com.
    SystemError,
    /// Payment method issuer doesn't allow this kind of payment because of limits on the account, or legal or compliance reasons.
    TransactionNotPermitted,
    /// Payment attempt unsuccessful, with no other information returned.
    Unknown,
}

/// Type of event sent by Paddle, in the format `entity.event_type`.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[non_exhaustive]
pub enum EventTypeName {
    /// An [`address.created`](https://developer.paddle.com/webhooks/addresses/address-created) event.
    AddressCreated,
    /// An [`address.imported`](https://developer.paddle.com/webhooks/addresses/address-imported) event.
    AddressImported,
    /// An [`address.updated`](https://developer.paddle.com/webhooks/addresses/address-updated) event.
    AddressUpdated,
    /// An [`adjustment.created`](https://developer.paddle.com/webhooks/adjustments/adjustment-created) event.
    AdjustmentCreated,
    /// An [`adjustment.updated`](https://developer.paddle.com/webhooks/adjustments/adjustment-updated) event.
    AdjustmentUpdated,
    /// A [`business.created`](https://developer.paddle.com/webhooks/businesses/business-created) event.
    BusinessCreated,
    /// A [`business.imported`](https://developer.paddle.com/webhooks/businesses/business-imported) event.
    BusinessImported,
    /// A [`business.updated`](https://developer.paddle.com/webhooks/businesses/business-updated) event.
    BusinessUpdated,
    /// A [`customer.created`](https://developer.paddle.com/webhooks/customers/customer-created) event.
    CustomerCreated,
    /// A [`customer.imported`](https://developer.paddle.com/webhooks/customers/customer-imported) event.
    CustomerImported,
    /// A [`customer.updated`](https://developer.paddle.com/webhooks/customers/customer-updated) event.
    CustomerUpdated,
    /// A [`discount.created`](https://developer.paddle.com/webhooks/discounts/discount-created) event.
    DiscountCreated,
    /// A [`discount.imported`](https://developer.paddle.com/webhooks/discounts/discount-imported) event.
    DiscountImported,
    /// A [`discount.updated`](https://developer.paddle.com/webhooks/discounts/discount-updated) event.
    DiscountUpdated,
    /// A [`payout.created`](https://developer.paddle.com/webhooks/payouts/payout-created) event.
    PayoutCreated,
    /// A [`payout.paid`](https://developer.paddle.com/webhooks/payouts/payout-paid) event.
    PayoutPaid,
    /// A [`price.created`](https://developer.paddle.com/webhooks/prices/price-created) event.
    PriceCreated,
    /// A [`price.imported`](https://developer.paddle.com/webhooks/prices/price-imported) event.
    PriceImported,
    /// A [`price.updated`](https://developer.paddle.com/webhooks/prices/price-updated) event.
    PriceUpdated,
    /// A [`product.created`](https://developer.paddle.com/webhooks/products/product-created) event.
    ProductCreated,
    /// A [`product.imported`](https://developer.paddle.com/webhooks/products/product-imported) event.
    ProductImported,
    /// A [`product.created`](https://developer.paddle.com/webhooks/products/product-updated) event.
    ProductUpdated,
    /// A [`report.created`](https://developer.paddle.com/webhooks/reports/report-created) event.
    ReportCreated,
    /// A [`report.updated`](https://developer.paddle.com/webhooks/reports/report-updated) event.
    ReportUpdated,
    /// A [`subscription.activated`](https://developer.paddle.com/webhooks/subscriptions/subscription-activated) event.
    SubscriptionActivated,
    /// A [`subscription.canceled`](https://developer.paddle.com/webhooks/subscriptions/subscription-canceled) event.
    SubscriptionCanceled,
    /// A [`subscription.created`](https://developer.paddle.com/webhooks/subscriptions/subscription-created) event.
    SubscriptionCreated,
    /// A [`subscription.imported`](https://developer.paddle.com/webhooks/subscriptions/subscription-imported) event.
    SubscriptionImported,
    /// A [`subscription.past_due`](https://developer.paddle.com/webhooks/subscriptions/subscription-past-due) event.
    SubscriptionPastDue,
    /// A [`subscription.paused`](https://developer.paddle.com/webhooks/subscriptions/subscription-paused) event.
    SubscriptionPaused,
    /// A [`subscription.resumed`](https://developer.paddle.com/webhooks/subscriptions/subscription-resumed) event.
    SubscriptionResumed,
    /// A [`subscription.trialing`](https://developer.paddle.com/webhooks/subscriptions/subscription-trialing) event.
    SubscriptionTrialing,
    /// A [`subscription.updated`](https://developer.paddle.com/webhooks/subscriptions/subscription-updated) event.
    SubscriptionUpdated,
    /// A [`transaction.billed`](https://developer.paddle.com/webhooks/transactions/transaction-billed) event.
    TransactionBilled,
    /// A [`transaction.canceled`](https://developer.paddle.com/webhooks/transactions/transaction-canceled) event.
    TransactionCanceled,
    /// A [`transaction.completed`](https://developer.paddle.com/webhooks/transactions/transaction-completed) event.
    TransactionCompleted,
    /// A [`transaction.created`](https://developer.paddle.com/webhooks/transactions/transaction-created) event.
    TransactionCreated,
    /// A [`transaction.paid`](https://developer.paddle.com/webhooks/transactions/transaction-paid) event.
    TransactionPaid,
    /// A [`transaction.past_due`](https://developer.paddle.com/webhooks/transactions/transaction-past-due) event.
    TransactionPastDue,
    /// A [`transaction.payment_failed`](https://developer.paddle.com/webhooks/transactions/transaction-payment-failed) event.
    TransactionPaymentFailed,
    /// A [`transaction.ready`](https://developer.paddle.com/webhooks/transactions/transaction-ready) event.
    TransactionReady,
    /// A [`transaction.updated`](https://developer.paddle.com/webhooks/transactions/transaction-updated) event.
    TransactionUpdated,
}

/// Type of event sent by Paddle along with it's corresponding entity data
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "event_type", content = "data")]
pub enum EventData {
    /// An [`address.created`](https://developer.paddle.com/webhooks/addresses/address-created) event.
    #[serde(rename = "address.created")]
    AddressCreated(crate::entities::Address),
    /// An [`address.imported`](https://developer.paddle.com/webhooks/addresses/address-imported) event.
    #[serde(rename = "address.imported")]
    AddressImported(crate::entities::Address),
    /// An [`address.updated`](https://developer.paddle.com/webhooks/addresses/address-updated) event.
    #[serde(rename = "address.updated")]
    AddressUpdated(crate::entities::Address),
    /// An [`adjustment.created`](https://developer.paddle.com/webhooks/adjustments/adjustment-created) event.
    #[serde(rename = "adjustment.created")]
    AdjustmentCreated(crate::entities::Adjustment),
    /// An [`adjustment.updated`](https://developer.paddle.com/webhooks/adjustments/adjustment-updated) event.
    #[serde(rename = "adjustment.updated")]
    AdjustmentUpdated(crate::entities::Adjustment),
    /// A [`api_key.created`](https://developer.paddle.com/webhooks/api-keys/api-key-created) event.
    #[serde(rename = "api_key.created")]
    ApiKeyCreated(crate::entities::ApiKey),
    /// A [`api_key.updated`](https://developer.paddle.com/webhooks/api-keys/api-key-updated) event.
    #[serde(rename = "api_key.updated")]
    ApiKeyUpdated(crate::entities::ApiKey),
    /// A [`api_key.expiring`](https://developer.paddle.com/webhooks/api-keys/api-key-expiring) event.
    #[serde(rename = "api_key.expiring")]
    ApiKeyExpiring(crate::entities::ApiKey),
    /// A [`api_key.expired`](https://developer.paddle.com/webhooks/api-keys/api-key-expired) event.
    #[serde(rename = "api_key.expired")]
    ApiKeyExpired(crate::entities::ApiKey),
    /// A [`api_key.revoked`](https://developer.paddle.com/webhooks/api-keys/api-key-revoked) event.
    #[serde(rename = "api_key.revoked")]
    ApiKeyRevoked(crate::entities::ApiKey),
    /// A [`business.created`](https://developer.paddle.com/webhooks/businesses/business-created) event.
    #[serde(rename = "business.created")]
    BusinessCreated(crate::entities::Business),
    /// A [`business.imported`](https://developer.paddle.com/webhooks/businesses/business-imported) event.
    #[serde(rename = "business.imported")]
    BusinessImported(crate::entities::Business),
    /// A [`business.updated`](https://developer.paddle.com/webhooks/businesses/business-updated) event.
    #[serde(rename = "business.updated")]
    BusinessUpdated(crate::entities::Business),
    /// A [`customer.created`](https://developer.paddle.com/webhooks/customers/customer-created) event.
    #[serde(rename = "customer.created")]
    CustomerCreated(crate::entities::Customer),
    /// A [`customer.imported`](https://developer.paddle.com/webhooks/customers/customer-imported) event.
    #[serde(rename = "customer.imported")]
    CustomerImported(crate::entities::Customer),
    /// A [`customer.updated`](https://developer.paddle.com/webhooks/customers/customer-updated) event.
    #[serde(rename = "customer.updated")]
    CustomerUpdated(crate::entities::Customer),
    /// A [`discount.created`](https://developer.paddle.com/webhooks/discounts/discount-created) event.
    #[serde(rename = "discount.created")]
    DiscountCreated(crate::entities::Discount),
    /// A [`discount.imported`](https://developer.paddle.com/webhooks/discounts/discount-imported) event.
    #[serde(rename = "discount.imported")]
    DiscountImported(crate::entities::Discount),
    /// A [`discount.updated`](https://developer.paddle.com/webhooks/discounts/discount-updated) event.
    #[serde(rename = "discount.updated")]
    DiscountUpdated(crate::entities::Discount),
    /// A [`payment_method.saved`](https://developer.paddle.com/webhooks/payment-methods/payment-method-saved) event.
    #[serde(rename = "payment_method.saved")]
    PaymentMethodSaved(crate::entities::PaymentMethod),
    /// A [`payment_method.deleted`](https://developer.paddle.com/webhooks/payment-methods/payment-method-deleted) event.
    #[serde(rename = "payment_method.deleted")]
    PaymentMethodDeleted(crate::entities::PaymentMethod),
    /// A [`payout.created`](https://developer.paddle.com/webhooks/payouts/payout-created) event.
    #[serde(rename = "payout.created")]
    PayoutCreated(crate::entities::Payout),
    /// A [`payout.paid`](https://developer.paddle.com/webhooks/payouts/payout-paid) event.
    #[serde(rename = "payout.paid")]
    PayoutPaid(crate::entities::Payout),
    /// A [`price.created`](https://developer.paddle.com/webhooks/prices/price-created) event.
    #[serde(rename = "price.created")]
    PriceCreated(crate::entities::Price),
    /// A [`price.imported`](https://developer.paddle.com/webhooks/prices/price-imported) event.
    #[serde(rename = "price.imported")]
    PriceImported(crate::entities::Price),
    /// A [`price.updated`](https://developer.paddle.com/webhooks/prices/price-updated) event.
    #[serde(rename = "price.updated")]
    PriceUpdated(crate::entities::Price),
    /// A [`product.created`](https://developer.paddle.com/webhooks/products/product-created) event.
    #[serde(rename = "product.created")]
    ProductCreated(crate::entities::Product),
    /// A [`product.imported`](https://developer.paddle.com/webhooks/products/product-imported) event.
    #[serde(rename = "product.imported")]
    ProductImported(crate::entities::Product),
    /// A [`product.updated`](https://developer.paddle.com/webhooks/products/product-updated) event.
    #[serde(rename = "product.updated")]
    ProductUpdated(crate::entities::Product),
    /// A [`report.created`](https://developer.paddle.com/webhooks/reports/report-created) event.
    #[serde(rename = "report.created")]
    ReportCreated(crate::entities::ReportBase),
    /// A [`report.updated`](https://developer.paddle.com/webhooks/reports/report-updated) event.
    #[serde(rename = "report.updated")]
    ReportUpdated(crate::entities::ReportBase),
    /// A [`subscription.activated`](https://developer.paddle.com/webhooks/subscriptions/subscription-activated) event.
    #[serde(rename = "subscription.activated")]
    SubscriptionActivated(crate::entities::Subscription),
    /// A [`subscription.canceled`](https://developer.paddle.com/webhooks/subscriptions/subscription-canceled) event.
    #[serde(rename = "subscription.canceled")]
    SubscriptionCanceled(crate::entities::Subscription),
    /// A [`subscription.created`](https://developer.paddle.com/webhooks/subscriptions/subscription-created) event.
    #[serde(rename = "subscription.created")]
    SubscriptionCreated(crate::entities::Subscription),
    /// A [`subscription.imported`](https://developer.paddle.com/webhooks/subscriptions/subscription-imported) event.
    #[serde(rename = "subscription.imported")]
    SubscriptionImported(crate::entities::Subscription),
    /// A [`subscription.past_due`](https://developer.paddle.com/webhooks/subscriptions/subscription-past-due) event.
    #[serde(rename = "subscription.past_due")]
    SubscriptionPastDue(crate::entities::Subscription),
    /// A [`subscription.paused`](https://developer.paddle.com/webhooks/subscriptions/subscription-paused) event.
    #[serde(rename = "subscription.paused")]
    SubscriptionPaused(crate::entities::Subscription),
    /// A [`subscription.resumed`](https://developer.paddle.com/webhooks/subscriptions/subscription-resumed) event.
    #[serde(rename = "subscription.resumed")]
    SubscriptionResumed(crate::entities::Subscription),
    /// A [`subscription.trialing`](https://developer.paddle.com/webhooks/subscriptions/subscription-trialing) event.
    #[serde(rename = "subscription.trialing")]
    SubscriptionTrialing(crate::entities::Subscription),
    /// A [`subscription.updated`](https://developer.paddle.com/webhooks/subscriptions/subscription-updated) event.
    #[serde(rename = "subscription.updated")]
    SubscriptionUpdated(crate::entities::Subscription),
    /// A [`transaction.billed`](https://developer.paddle.com/webhooks/transactions/transaction-billed) event.
    #[serde(rename = "transaction.billed")]
    TransactionBilled(crate::entities::Transaction),
    /// A [`transaction.canceled`](https://developer.paddle.com/webhooks/transactions/transaction-canceled) event.
    #[serde(rename = "transaction.canceled")]
    TransactionCanceled(crate::entities::Transaction),
    /// A [`transaction.completed`](https://developer.paddle.com/webhooks/transactions/transaction-completed) event.
    #[serde(rename = "transaction.completed")]
    TransactionCompleted(crate::entities::Transaction),
    /// A [`transaction.created`](https://developer.paddle.com/webhooks/transactions/transaction-created) event.
    #[serde(rename = "transaction.created")]
    TransactionCreated(crate::entities::Transaction),
    /// A [`transaction.paid`](https://developer.paddle.com/webhooks/transactions/transaction-paid) event.
    #[serde(rename = "transaction.paid")]
    TransactionPaid(crate::entities::Transaction),
    /// A [`transaction.past_due`](https://developer.paddle.com/webhooks/transactions/transaction-past-due) event.
    #[serde(rename = "transaction.past_due")]
    TransactionPastDue(crate::entities::Transaction),
    /// A [`transaction.payment_failed`](https://developer.paddle.com/webhooks/transactions/transaction-payment-failed) event.
    #[serde(rename = "transaction.payment_failed")]
    TransactionPaymentFailed(crate::entities::Transaction),
    /// A [`transaction.ready`](https://developer.paddle.com/webhooks/transactions/transaction-ready) event.
    #[serde(rename = "transaction.ready")]
    TransactionReady(crate::entities::Transaction),
    /// A [`transaction.revised`](https://developer.paddle.com/webhooks/transactions/transaction-revised) event.
    #[serde(rename = "transaction.revised")]
    TransactionRevised(crate::entities::Transaction),
    /// A [`transaction.updated`](https://developer.paddle.com/webhooks/transactions/transaction-updated) event.
    #[serde(rename = "transaction.updated")]
    TransactionUpdated(crate::entities::Transaction),
}

/// Status of this subscription item. Set automatically by Paddle.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "strum", strum(serialize_all = "lowercase"))]
#[non_exhaustive]
pub enum SubscriptionItemStatus {
    /// This item is active. It is not in trial and Paddle bills for it.
    Active,
    /// This item is not active. Set when the related subscription is paused.
    Inactive,
    /// This item is in trial. Paddle has not billed for it.
    Trialing,
}

/// How tax is calculated for this price.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum TaxMode {
    /// Prices use the setting from your account.
    AccountSetting,
    /// Prices are exclusive of tax.
    External,
    /// Prices are inclusive of tax.
    Internal,
}

/// Tax category for this product. Used for charging the correct rate of tax. Selected tax category must be enabled on your Paddle account.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "kebab-case"))]
#[non_exhaustive]
pub enum TaxCategory {
    /// Non-customizable digital files or media (not software) acquired with an up front payment that can be accessed without any physical product being delivered.
    DigitalGoods,
    /// Digital books and educational material which is sold with permanent rights for use by the customer.
    Ebooks,
    /// Remote configuration, set-up, and integrating software on behalf of a customer.
    ImplementationServices,
    /// Services that involve the application of your expertise and specialized knowledge of a software product.
    ProfessionalServices,
    /// Products that allow users to connect to and use online or cloud-based applications over the Internet.
    Saas,
    /// Services that can be used to customize and white label software products.
    SoftwareProgrammingServices,
    /// Software products that are pre-written and can be downloaded and installed onto a local device.
    Standard,
    /// Training and education services related to software products.
    TrainingServices,
    /// Cloud storage service for personal or corporate information, assets, or intellectual property.
    WebsiteHosting,
}

impl AsRef<str> for TaxCategory {
    fn as_ref(&self) -> &str {
        match self {
            Self::DigitalGoods => "digital-goods",
            Self::Ebooks => "ebooks",
            Self::ImplementationServices => "implementation-services",
            Self::ProfessionalServices => "professional-services",
            Self::Saas => "saas",
            Self::SoftwareProgrammingServices => "software-programming-services",
            Self::Standard => "standard",
            Self::TrainingServices => "training-services",
            Self::WebsiteHosting => "website-hosting",
        }
    }
}

/// Type of payment method used for this payment attempt.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum PaymentMethodType {
    /// Alipay, popular in China.
    Alipay,
    /// Apple Pay on a supported Apple device.
    ApplePay,
    /// Bancontact, popular in Belgium.
    Bancontact,
    /// Credit or debit card.
    Card,
    /// Google Pay on a supported Android device, Chromebook, or Google Chrome browser.
    GooglePay,
    /// iDEAL, popular in the Netherlands.
    Ideal,
    /// Payment recorded offline.
    Offline,
    /// PayPal.
    Paypal,
    /// Payment method not known.
    Unknown,
    /// Wire transfer, sometimes called bank transfer.
    WireTransfer,
}

/// Status of this notification.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[non_exhaustive]
pub enum NotificationStatus {
    /// Paddle hasn't yet tried to deliver this notification.
    NotAttempted,
    /// Paddle tried to deliver this notification, but it failed. It's scheduled to be retried.
    NeedsRetry,
    /// Paddle delivered this notification successfully.
    Delivered,
    /// Paddle tried to deliver this notification, but all attempts failed. It's not scheduled to be retried.
    Failed,
}

/// Describes how this notification was created.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[non_exhaustive]
pub enum NotificationOrigin {
    /// Notification created when a subscribed event occurred.
    Event,
    /// Notification created when a notification with the origin `event` was replayed.
    Replay,
}

/// Where notifications should be sent for this destination.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[non_exhaustive]
pub enum NotificationSettingType {
    /// Deliver to an email address.
    Email,
    /// Deliver to a webhook endpoint.
    Url,
}

/// Whether Paddle should deliver real platform events, simulation events or both to this notification destination.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[non_exhaustive]
pub enum TrafficSource {
    /// Deliver real platform events to this notification destination.
    Platform,
    /// Deliver simulation events to this notification destination.
    Simulation,
    /// Deliver platform and simulation events to this notification destination.
    All,
}

/// Operator to use when filtering.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "strum", strum(serialize_all = "lowercase"))]
#[non_exhaustive]
pub enum FilterOperator {
    /// Less than.
    Lt,
    /// Greater than or equal to.
    Gte,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum TransactionOrigin {
    /// Transaction created via the Paddle API.
    Api,
    /// Transaction created automatically by Paddle as a result of a one-time charge for a subscription.
    SubscriptionCharge,
    /// Transaction created automatically as part of updating a payment method. May be a zero value transaction.
    SubscriptionPaymentMethodChange,
    /// Transaction created automatically by Paddle as a result of a subscription renewal.
    SubscriptionRecurring,
    /// Transaction created automatically by Paddle as a result of an update to a subscription.
    SubscriptionUpdate,
    /// Transaction created automatically by Paddle.js for a checkout.
    Web,
}

/// Status of this report. Set automatically by Paddle.
///
/// Reports are created as `pending` initially, then move to `ready` when they're available to download.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum ReportStatus {
    /// Report created, but Paddle is processing it. It's not yet ready for download.
    Pending,
    /// Report fully processed by Paddle and ready for download.
    Ready,
    /// There was a problem processing this report.
    Failed,
    /// Report has expired and is no longer accessible.
    Expired,
}

/// Field name to filter by.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum AdjustmentsReportFilterName {
    /// Filter by adjustment action. Pass an array of strings containing any valid value for the `action` field against an adjustment.
    Action,
    /// Filter by transaction or adjustment currency. Pass an array of strings containing any valid supported three-letter ISO 4217 currency code.
    CurrencyCode,
    /// Filter by transaction or adjustment status. Pass an array of strings containing any valid value for the `status` field against a transaction or an adjustment.
    Status,
    /// Filter by transaction or adjustment updated date. Pass an RFC 3339 datetime string.
    UpdatedAt,
}

/// Field name to filter by.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum DiscountsReportFilterName {
    /// Filter by discount type. Pass an array of strings containing any valid value for the `type` field against a discount.
    Type,
    /// Filter by discount status. Pass an array of strings containing any valid value for the `status` field against a discount.
    Status,
    /// Filter by discount updated date. Pass an RFC 3339 datetime string.
    UpdatedAt,
}

/// Field name to filter by.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum BalanceReportFilterName {
    /// Filter by discount updated date. Pass an RFC 3339 datetime string.
    UpdatedAt,
}

/// Field name to filter by.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum ProductPricesReportFilterName {
    /// Filter by product status. Pass an array of strings containing any valid value for the `status` field against a product.
    ProductStatus,
    /// Filter by price status. Pass an array of strings containing any valid value for the `status` field against a price.
    PriceStatus,
    /// Filter by product type. Pass an array of strings containing any valid value for the `type` field against a product.
    ProductType,
    /// Filter by price type. Pass an array of strings containing any valid value for the `type` field against a price.
    PriceType,
    /// Filter by product `updated_at` date. Pass an RFC 3339 datetime string.
    ProductUpdatedAt,
    /// Filter by price `updated_at` date. Pass an RFC 3339 datetime string.
    PriceUpdatedAt,
}

/// Field name to filter by.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum TransactionsReportFilterName {
    /// Filter by collection mode. Pass an array of strings containing any valid value for the `collection_mode` field against a transaction.
    CollectionMode,
    /// Filter by transaction or adjustment currency. Pass an array of strings containing any valid supported three-letter ISO 4217 currency code.
    CurrencyCode,
    /// Filter by transaction origin. Pass an array of strings containing any valid value for the origin field against a transaction.
    Origin,
    /// Filter by transaction or adjustment status. Pass an array of strings containing any valid value for the `status` field against a transaction or an adjustment.
    Status,
    /// Filter by transaction or adjustment updated date. Pass an RFC 3339 datetime string.
    UpdatedAt,
}

/// Type of report.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum AdjustmentsReportType {
    /// Adjustments reports contain information about refunds, credits, and chargebacks.
    Adjustments,
    /// Adjustments reports contain information about refunds, credits, and chargebacks. The report is broken down by line item level.
    AdjustmentLineItems,
}

impl ReportType for AdjustmentsReportType {
    type FilterName = AdjustmentsReportFilterName;
}

/// Type of report.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum TransactionsReportType {
    /// Transactions reports contain information about revenue received, past due invoices, draft and issued invoices, and canceled transactions.
    Transactions,
    /// Transactions reports contain information about revenue received, past due invoices, draft and issued invoices, and canceled transactions. The report is broken down by line item level.
    TransactionLineItems,
}

impl ReportType for TransactionsReportType {
    type FilterName = TransactionsReportFilterName;
}

/// Type of report.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum ProductsAndPricesReportType {
    /// Products and prices reports contain information about your products and prices. May include non-catalog products and prices.
    ProductsPrices,
}

impl ReportType for ProductsAndPricesReportType {
    type FilterName = ProductPricesReportFilterName;
}

/// Type of report.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum DiscountsReportType {
    /// Discounts reports contain information about your product and checkout discounts.
    Discounts,
}

impl ReportType for DiscountsReportType {
    type FilterName = DiscountsReportFilterName;
}

/// Type of report.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum BalanceReportType {
    /// Balance reports contain information about your account balance activity, including all movements of funds in and out of your balance.
    Balance,
}

impl ReportType for BalanceReportType {
    type FilterName = BalanceReportFilterName;
}

/// Status of this simulation run log.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[non_exhaustive]
pub enum SimulationEventStatus {
    /// Simulation run log is pending. Paddle hasn't yet tried to deliver the simulated event.
    Pending,
    /// Simulation run log was successful. Paddle delivered the simulated event successfully.
    Success,
    /// Simulation run log failed. Paddle tried to deliver the simulated event, but it failed. If `response` object is `null`, no response received from your server. Check your notification setting endpoint configuration.
    Failed,
    /// Simulation run log aborted. Paddle could not attempt delivery of the simulated event.
    Aborted,
}

/// Status of this simulation run.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[non_exhaustive]
pub enum SimulationRunStatus {
    /// Simulation run is pending. Paddle is sending events that are part of this simulation.
    Pending,
    /// Simulation run is completed. Paddle attempted to send events that are part of this simulation.
    Completed,
    /// Simulation run is canceled. Simulation run was canceled before all events were sent.
    Canceled,
}

/// Scenario for a simulation.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[non_exhaustive]
pub enum SimulationScenarioType {
    /// Simulates all events sent when a subscription is created.
    SubscriptionCreation,
    /// Simulates all events sent when a subscription is renewed.
    SubscriptionRenewal,
    /// Simulates all events sent when a subscription is paused.
    SubscriptionPause,
    /// Simulates all events sent when a subscription is resumed.
    SubscriptionResume,
    /// Simulates all events sent when a subscription is canceled.
    SubscriptionCancellation,
}

/// Type of simulation.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[non_exhaustive]
pub enum SimulationKind {
    /// Paddle simulates a single event.
    SingleEvent,
    /// Paddle simulates a predefined series of events for a scenario, like all events created when a subscription renews.
    Scenario,
}

/// Status of this payment attempt.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum PaymentAttemptStatus {
    /// Authorized but not captured. Payment attempt is incomplete.
    Authorized,
    /// Authorized but not captured because it has been flagged as potentially fraudulent. Payment attempt is incomplete.
    AuthorizedFlagged,
    /// Previously authorized payment attempt has been canceled. Typically when `authorized_flagged` payment attempts are rejected.
    Canceled,
    /// Payment captured successfully. Payment attempt is complete.
    Captured,
    /// Something went wrong and the payment attempt was unsuccessful. Check the `error_code` for more information.
    Error,
    /// Customer must complete an action for this payment attempt to proceed. Typically means that the payment attempt requires 3DS.
    ActionRequired,
    /// Response required from the bank or payment provider. Transaction is pending.
    PendingNoActionRequired,
    /// New payment attempt created.
    Created,
    /// Payment attempt status not known.
    Unknown,
    /// Payment attempt dropped by Paddle.
    Dropped,
}

/// Status of this subscription. Set automatically by Paddle. Use the pause subscription or cancel subscription operations to change.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum SubscriptionStatus {
    /// Subscription is active. Paddle is billing for this subscription and related transactions aren't past due.
    Active,
    /// Subscription is canceled. Automatically set by Paddle when a subscription is canceled. When a subscription is set to cancel on the next billing period, a scheduled change for the cancellation is created. The subscription status moves to canceled when the scheduled change takes effect.
    Canceled,
    /// Subscription has an overdue payment. Automatically set by Paddle when payment fails for an automatically-collected transaction, or when payment terms have elapsed for a manually-collected transaction (an invoice).
    PastDue,
    /// Subscription is paused. Automatically set by Paddle when a subscription is paused. When a subscription is set to pause on the next billing period, a scheduled change for the pause is created. The subscription status moves to `paused` when the scheduled change takes effect.
    Paused,
    /// Subscription is in trial.
    Trialing,
}

/// Status of this transaction. You may set a transaction to `billed` or `canceled`, other statuses are set automatically by Paddle. Automatically-collected transactions may return `completed` if payment is captured successfully, or `past_due` if payment failed.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum TransactionStatus {
    /// Transaction is missing required fields. Typically the first stage of a checkout before customer details are captured.
    Draft,
    /// Transaction has all of the required fields to be marked as `billed` or `completed`.
    Ready,
    /// Transaction has been updated to `billed`. Billed transactions get an invoice number and are considered a legal record. They cannot be changed. Typically used as part of an invoice workflow.
    Billed,
    /// Transaction is fully paid, but has not yet been processed internally.
    Paid,
    /// Transaction is fully paid and processed.
    Completed,
    /// Transaction has been updated to `canceled`. If an invoice, it's no longer due.
    Canceled,
    /// Transaction is past due. Occurs for automatically-collected transactions when the related subscription is in dunning, and for manually-collected transactions when payment terms have elapsed.
    PastDue,
}

/// Kind of change that's scheduled to be applied to this subscription.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "strum", strum(serialize_all = "lowercase"))]
#[non_exhaustive]
pub enum ScheduledChangeAction {
    /// Subscription is scheduled to cancel. Its status changes to `canceled` on the `effective_at` date.
    Cancel,
    /// Subscription is scheduled to pause. Its status changes to `pause` on the `effective_at` date.
    Pause,
    /// Subscription is scheduled to resume. Its status changes to `active` on the `resume_at` date.
    Resume,
}

/// How Paddle should handle changes made to a subscription or its items if the payment fails during update. If omitted, defaults to `prevent_change`.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum SubscriptionOnPaymentFailure {
    /// In case of payment failure, prevent the change to the subscription from applying.
    PreventChange,
    /// In case of payment failure, apply the change and update the subscription.
    ApplyChange,
}

/// Whether the subscription change results in a prorated credit or a charge.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "strum", strum(serialize_all = "lowercase"))]
#[non_exhaustive]
pub enum UpdateSummaryResultAction {
    /// Changes to the subscription results in a prorated credit.
    Credit,
    /// Changes to the subscription results in a prorated charge.
    Charge,
}

/// How Paddle should handle proration calculation for changes made to a subscription or its items. Required when making
/// changes that impact billing.
///
/// For automatically-collected subscriptions, responses may take longer than usual if a proration billing mode that
/// collects for payment immediately is used.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum ProrationBillingMode {
    /// Paddle calculates the prorated amount for the subscription changes based on the current billing cycle, then
    /// creates a transaction to collect immediately.
    ProratedImmediately,
    /// Paddle calculates the prorated amount for the subscription changes based on the current billing cycle, then
    /// schedules them to be billed on the next renewal.
    ProratedNextBillingPeriod,
    /// Paddle does not calculate proration for the subscription changes, creating a transaction to collect for the full
    /// amount immediately.
    FullImmediately,
    /// Paddle does not calculate proration for the subscription changes, scheduling for the full amount for the changes
    /// to be billed on the next renewal.
    FullNextBillingPeriod,
    /// Paddle does not bill for the subscription changes.
    DoNotBill,
}

/// How Paddle should set the billing period for the subscription when resuming. If omitted, defaults to `start_new_billing_period`.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum SubscriptionOnResume {
    /// When resuming, continue the existing billing period. If the customer resumes before the end date of the existing billing period, there's no immediate charge. If after, an error is returned.
    ContinueExistingBillingPeriod,
    /// When resuming, start a new billing period. The `current_billing_period.starts_at` date is set to the resume date, and Paddle immediately charges the full amount for the new billing period.
    StartNewBillingPeriod,
}

/// Determine whether the generated URL should download the PDF as an attachment saved locally, or open it inline in the browser.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "strum", strum(serialize_all = "lowercase"))]
#[non_exhaustive]
pub enum Disposition {
    /// Generated URL downloads the PDF as an attachment. Browsers typically automatically save the PDF.
    Attachment,
    /// Generated URL displays the PDF inline in the browser. Browsers typically open the PDF in the current tab.
    Inline,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "strum", strum(serialize_all = "lowercase"))]
#[non_exhaustive]
pub enum PayoutStatus {
    /// Payout is paid.
    Paid,
    /// Payout is unpaid. Typically means it has been created, but is not yet completed.
    Unpaid,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "strum", strum(serialize_all = "lowercase"))]
#[non_exhaustive]
pub enum ApiKeyStatus {
    Active,
    Expired,
    Revoked,
}

/// Include related entities in the response.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "strum", derive(EnumString, Display))]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
#[non_exhaustive]
pub enum SubscriptionInclude {
    /// Include an object with a preview of the next transaction for this subscription. May include prorated charges that aren't yet billed and one-time charges.
    NextTransaction,
    /// Include an object with a preview of the recurring transaction for this subscription. This is what the customer can expect to be billed when there are no prorated or one-time charges.
    RecurringTransactionDetails,
}
