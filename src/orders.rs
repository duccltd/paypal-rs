//! An order represents a payment between two or more parties.
//!
//! Use the Orders API to create, update, retrieve, authorize, and capture orders.
//!
//! Reference: https://developer.paypal.com/docs/api/orders/v2/

use crate::common::*;
use crate::errors::{PaypalError, ResponseError};
use crate::client::{HeaderParams, Client};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The intent to either capture payment immediately or authorize a payment for an order after order creation.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Intent {
    /// The merchant intends to capture payment immediately after the customer makes a payment.
    Capture,
    /// The merchant intends to authorize a payment and place funds on hold after the customer makes a payment.
    /// Authorized payments are guaranteed for up to three days but are available to capture for up to 29 days.
    /// After the three-day honor period, the original authorized payment expires and you must re-authorize the payment.
    /// You must make a separate request to capture payments on demand.
    /// This intent is not supported when you have more than one `purchase_unit` within your order.
    Authorize,
}

impl Default for Intent {
    fn default() -> Self {
        Intent::Capture
    }
}

/// Represents a payer name.
///
/// https://developer.paypal.com/docs/api/orders/v2/#definition-payer.name
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct PayerName {
    /// When the party is a person, the party's given, or first, name.
    pub given_name: String,
    /// When the party is a person, the party's surname or family name. Also known as the last name.
    /// Required when the party is a person. Use also to store multiple surnames including the matronymic, or mother's, surname.
    pub surname: String,
}

/// The phone number, in its canonical international E.164 numbering plan format.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct PhoneNumber {
    /// The national number, in its canonical international E.164 numbering plan format.
    /// The combined length of the country calling code (CC) and the national number must not be greater than 15 digits.
    /// The national number consists of a national destination code (NDC) and subscriber number (SN).
    pub national_number: String,
}

/// The phone number of the customer. Available only when you enable the
/// Contact Telephone Number option in the Profile & Settings for the merchant's PayPal account.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Phone {
    /// The phone type.
    pub phone_type: Option<PhoneType>,
    /// The phone number
    pub phone_number: PhoneNumber,
}

/// The customer's tax ID type. Supported for the PayPal payment method only.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(non_camel_case_types)]
pub enum TaxIdType {
    /// The individual tax ID type.
    BR_CPF,
    /// The business tax ID type.
    BR_CNPJ,
}

/// The tax information of the payer.
#[derive(Debug, Serialize, Deserialize)]
pub struct TaxInfo {
    /// The customer's tax ID. Supported for the PayPal payment method only.
    /// Typically, the tax ID is 11 characters long for individuals and 14 characters long for businesses.
    pub tax_id: String,
    /// The customer's tax ID type. Supported for the PayPal payment method only.
    pub tax_id_type: TaxIdType,
}

/// The customer who approves and pays for the order. The customer is also known as the payer.
///
/// https://developer.paypal.com/docs/api/orders/v2/#definition-payer
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Payer {
    /// The name of the payer.
    pub name: Option<PayerName>,
    /// The email address of the payer.
    pub email_address: Option<String>,
    /// The PayPal-assigned ID for the payer.
    pub payer_id: Option<String>,
    /// The phone number of the customer. Available only when you enable the Contact
    /// Telephone Number option in the Profile & Settings for the merchant's PayPal account.
    pub phone: Option<Phone>,
    /// The birth date of the payer in YYYY-MM-DD format.
    pub birth_date: Option<String>,
    /// The tax information of the payer. Required only for Brazilian payer's.
    pub tax_info: Option<TaxInfo>,
    /// The address of the payer.
    pub address: Option<Address>,
}

/// Breakdown provides details such as total item amount, total tax amount, shipping, handling, insurance, and discounts, if any.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Breakdown {
    /// The subtotal for all items. Required if the request includes purchase_units[].items[].unit_amount.
    /// Must equal the sum of (items[].unit_amount * items[].quantity) for all items.
    pub item_total: Option<Money>,
    /// The shipping fee for all items within a given purchase_unit.
    pub shipping: Option<Money>,
    /// The handling fee for all items within a given purchase_unit.
    pub handling: Option<Money>,
    /// The total tax for all items. Required if the request includes purchase_units.items.tax. Must equal the sum of (items[].tax * items[].quantity) for all items.
    pub tax_total: Option<Money>,
    /// The insurance fee for all items within a given purchase_unit.
    pub insurance: Option<Money>,
    /// The shipping discount for all items within a given purchase_unit.
    pub shipping_discount: Option<Money>,
    /// The discount for all items within a given purchase_unit.
    pub discount: Option<Money>,
}

/// Represents an amount of money.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Amount {
    /// The [three-character ISO-4217 currency code](https://developer.paypal.com/docs/integration/direct/rest/currency-codes/) that identifies the currency.
    pub currency_code: Currency,
    /// The value, which might be:
    /// - An integer for currencies like JPY that are not typically fractional.
    /// - A decimal fraction for currencies like TND that are subdivided into thousandths.
    ///
    /// For the required number of decimal places for a currency code, see [Currency Codes](https://developer.paypal.com/docs/api/reference/currency-codes/).
    pub value: String,
    /// The breakdown of the amount.
    pub breakdown: Option<Breakdown>,
}

impl Amount {
    /// Creates a new amount with the required values.
    pub fn new(currency: Currency, value: &str) -> Self {
        Amount {
            currency_code: currency,
            value: value.to_owned(),
            breakdown: None,
        }
    }
}

/// The merchant who receives payment for this transaction.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Payee {
    /// The email address of merchant.
    pub email_address: Option<String>,
    /// The encrypted PayPal account ID of the merchant.
    pub merchant_id: Option<String>,
}

/// Fees, commissions, tips, or donations
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformFee {
    /// The fee for this transaction.
    pub amount: Money,

    /// The merchant who receives payment for this transaction.
    pub payee: Option<Payee>,
}

/// The funds that are held on behalf of the merchant
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum DisbursementMode {
    /// The funds are released to the merchant immediately.
    Instant,
    /// The funds are held for a finite number of days. The actual duration depends on the region and type of integration.
    /// You can release the funds through a referenced payout.
    /// Otherwise, the funds disbursed automatically after the specified duration.
    Delayed,
}

impl Default for DisbursementMode {
    fn default() -> Self {
        DisbursementMode::Instant
    }
}

/// Any additional payment instructions for PayPal Commerce Platform customers.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PaymentInstruction {
    /// An array of various fees, commissions, tips, or donations.
    pub platform_fees: Option<Vec<PlatformFee>>,
    /// The funds that are held on behalf of the merchant.
    pub disbursement_mode: Option<DisbursementMode>,
}

/// The item category type.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ItemCategoryType {
    /// Goods that are stored, delivered, and used in their electronic format.
    /// This value is not currently supported for API callers that leverage
    /// the [PayPal for Commerce Platform](https://www.paypal.com/us/webapps/mpp/commerce-platform) product.
    Digital,
    /// A tangible item that can be shipped with proof of delivery.
    Physical,
}

impl Default for ItemCategoryType {
    fn default() -> Self {
        ItemCategoryType::Digital
    }
}

/// The name and address of the person to whom to ship the items.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ShippingDetail {
    /// The name of the person to whom to ship the items. Supports only the full_name property.
    pub name: Option<String>,
    /// The address of the person to whom to ship the items.
    pub address: Option<Address>,
}

/// Represents an item.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    /// The item name or title.
    pub name: String,
    /// The item price or rate per unit.
    /// If you specify unit_amount, purchase_units[].amount.breakdown.item_total is required. Must equal unit_amount * quantity for all items.
    pub unit_amount: Money,
    /// The item tax for each unit. If tax is specified, purchase_units[].amount.breakdown.tax_total is required. Must equal tax * quantity for all items.
    pub tax: Option<Money>,
    /// The item quantity. Must be a whole number.
    pub quantity: String,
    /// The detailed item description.
    pub description: Option<String>,
    /// The stock keeping unit (SKU) for the item.
    pub sku: Option<String>,
    /// The item category type
    pub category: Option<ItemCategoryType>,
}

/// The status of the payment authorization.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthorizationStatus {
    /// The authorized payment is created. No captured payments have been made for this authorized payment.
    Created,
    /// The authorized payment has one or more captures against it. The sum of these captured payments is greater than the amount of the original authorized payment.
    Captured,
    /// PayPal cannot authorize funds for this authorized payment.
    Denied,
    /// The authorized payment has expired.
    Expired,
    /// A captured payment was made for the authorized payment for an amount that is less than the amount of the original authorized payment.
    PartiallyExpired,
    /// The payment which was authorized for an amount that is less than the originally requested amount.
    PartiallyCaptured,
    /// The authorized payment was voided. No more captured payments can be made against this authorized payment.
    Voided,
    /// The created authorization is in pending state. For more information, see status.details.
    Pending,
}

/// Authorization status reason.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthorizationStatusDetailsReason {
    /// Authorization is pending manual review.
    PendingReview,
}

/// Details about the status of the authorization.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct AuthorizationStatusDetails {
    /// The reason why the authorized status is PENDING.
    pub reason: AuthorizationStatusDetailsReason,
}

/// A payment authorization.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct AuthorizationWithData {
    /// The status for the authorized payment.
    pub status: AuthorizationStatus,
    /// The details of the authorized order pending status.
    pub status_details: AuthorizationStatusDetails,
}

/// The capture status.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CaptureStatus {
    /// The funds for this captured payment were credited to the payee's PayPal account.
    Completed,
    ///  The funds could not be captured.
    Declined,
    /// An amount less than this captured payment's amount was partially refunded to the payer.
    PartiallyRefunded,
    /// The funds for this captured payment was not yet credited to the payee's PayPal account. For more information, see status.details.
    Pending,
    /// An amount greater than or equal to this captured payment's amount was refunded to the payer.
    Refunded,
}

/// Capture status reason.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CaptureStatusDetailsReason {
    /// The payer initiated a dispute for this captured payment with PayPal.
    BuyerComplaint,
    /// The captured funds were reversed in response to the payer disputing this captured payment with
    /// the issuer of the financial instrument used to pay for this captured payment.
    Chargeback,
    /// The payer paid by an eCheck that has not yet cleared.
    Echeck,
    /// Visit your online account. In your **Account Overview**, accept and deny this payment.
    InternationalWithdrawal,
    /// No additional specific reason can be provided. For more information about this captured payment, visit your account online or contact PayPal.
    Other,
    /// The captured payment is pending manual review.
    PendingReview,
    /// The payee has not yet set up appropriate receiving preferences for their account.
    /// For more information about how to accept or deny this payment, visit your account online.
    /// This reason is typically offered in scenarios such as when the currency of the captured
    /// payment is different from the primary holding currency of the payee.
    ReceivingPreferenceMandatesManualAction,
    /// The captured funds were refunded.
    Refunded,
    /// The payer must send the funds for this captured payment. This code generally appears for manual EFTs.
    TransactionApprovedAwaitingFunding,
    /// The payee does not have a PayPal account.
    Unilateral,
    /// The payee's PayPal account is not verified.
    VerificationRequired,
}

/// Details about the captured payment status.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct CaptureStatusDetails {
    /// The reason why the captured payment status is PENDING or DENIED.
    pub reason: CaptureStatusDetailsReason,
}

/// A captured payment.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Capture {
    /// The status of the captured payment.
    pub status: CaptureStatus,
    /// The details of the captured payment status.
    pub status_details: Option<CaptureStatusDetails>,
}

/// The status of the refund
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RefundStatus {
    /// The refund was cancelled.
    Cancelled,
    /// The refund is pending. For more information, see status_details.reason.
    Pending,
    /// The funds for this transaction were debited to the customer's account.
    Completed,
}

/// Refund status reason.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RefundStatusDetailsReason {
    /// The customer's account is funded through an eCheck, which has not yet cleared.
    Echeck,
}

/// Details about the status of the refund.
#[derive(Debug, Serialize, Deserialize)]
pub struct RefundStatusDetails {
    /// The reason why the refund has the PENDING or FAILED status.
    pub reason: RefundStatusDetailsReason,
}

/// A refund
#[derive(Debug, Serialize, Deserialize)]
pub struct Refund {
    /// The status of the refund.
    pub status: RefundStatus,
    /// The details of the refund status.
    pub status_details: RefundStatusDetails,
}

/// The comprehensive history of payments for the purchase unit.
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentCollection {
    /// An array of authorized payments for a purchase unit. A purchase unit can have zero or more authorized payments.
    #[serde(default)]
    pub authorizations: Vec<AuthorizationWithData>,
    /// An array of captured payments for a purchase unit. A purchase unit can have zero or more captured payments.
    #[serde(default)]
    pub captures: Vec<Capture>,
    /// An array of refunds for a purchase unit. A purchase unit can have zero or more refunds.
    #[serde(default)]
    pub refunds: Vec<Refund>,
}

/// Represents either a full or partial order that the payer intends to purchase from the payee.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PurchaseUnit {
    /// The API caller-provided external ID for the purchase unit. Required for multiple purchase units when you must update the order through PATCH.
    /// If you omit this value and the order contains only one purchase unit, PayPal sets this value to default.
    pub reference_id: Option<String>,
    /// The total order amount with an optional breakdown that provides details, such as the total item amount,
    /// total tax amount, shipping, handling, insurance, and discounts, if any.
    ///
    /// If you specify amount.breakdown, the amount equals item_total plus tax_total plus shipping plus handling plus insurance minus shipping_discount minus discount.
    ///
    /// The amount must be a positive number. For listed of supported currencies and decimal precision,
    /// see the PayPal REST APIs [Currency Codes](https://developer.paypal.com/docs/integration/direct/rest/currency-codes/).
    pub amount: Amount,
    /// The merchant who receives payment for this transaction.
    pub payee: Option<Payee>,
    /// Any additional payment instructions for PayPal Commerce Platform customers.
    /// Enables features for the PayPal Commerce Platform, such as delayed disbursement and collection of a platform fee.
    /// Applies during order creation for captured payments or during capture of authorized payments.
    pub payment_instruction: Option<PaymentInstruction>,
    /// The purchase description.
    pub description: Option<String>,
    /// The API caller-provided external ID. Used to reconcile client transactions with PayPal transactions.
    /// Appears in transaction and settlement reports but is not visible to the payer.
    pub custom_id: Option<String>,
    /// The API caller-provided external invoice number for this order.
    /// Appears in both the payer's transaction history and the emails that the payer receives.
    pub invoice_id: Option<String>,
    /// The PayPal-generated ID for the purchase unit.
    /// This ID appears in both the payer's transaction history and the emails that the payer receives.
    /// In addition, this ID is available in transaction and settlement reports that merchants and API callers can use to reconcile transactions.
    /// This ID is only available when an order is saved by calling v2/checkout/orders/id/save.
    pub id: Option<String>,
    /// The soft descriptor is the dynamic text used to construct the statement descriptor that appears on a payer's card statement.
    ///
    /// More info here: https://developer.paypal.com/docs/api/orders/v2/#definition-purchase_unit_request
    pub soft_descriptor: Option<String>,
    /// An array of items that the customer purchases from the merchant.
    pub items: Option<Vec<Item>>,
    /// The name and address of the person to whom to ship the items.
    pub shipping: Option<ShippingDetail>,
    /// The comprehensive history of payments for the purchase unit.
    pub payments: Option<PaymentCollection>,
}

impl PurchaseUnit {
    /// Creates a new PurchaseUnit with the required properties.
    pub fn new(amount: Amount) -> Self {
        Self {
            amount,
            ..Default::default()
        }
    }
}

/// The type of landing page to show on the PayPal site for customer checkout.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LandingPage {
    /// When the customer clicks PayPal Checkout, the customer is redirected to a page to log in to PayPal and approve the payment.
    Login,
    /// When the customer clicks PayPal Checkout, the customer is redirected to a page
    /// to enter credit or debit card and other relevant billing information required to complete the purchase.
    Billing,
    /// When the customer clicks PayPal Checkout, the customer is redirected to either a page to log in to PayPal and approve
    /// the payment or to a page to enter credit or debit card and other relevant billing information required to complete the purchase,
    /// depending on their previous interaction with PayPal.
    NoPreference,
}

impl Default for LandingPage {
    fn default() -> Self {
        LandingPage::NoPreference
    }
}

/// The shipping preference
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShippingPreference {
    /// Use the customer-provided shipping address on the PayPal site.
    GetFromFile,
    /// Redact the shipping address from the PayPal site. Recommended for digital goods.
    NoShipping,
    ///  Use the merchant-provided address. The customer cannot change this address on the PayPal site.
    SetProvidedAddress,
}

impl Default for ShippingPreference {
    fn default() -> Self {
        ShippingPreference::GetFromFile
    }
}

/// Configures a Continue or Pay Now checkout flow.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserAction {
    /// After you redirect the customer to the PayPal payment page, a Continue button appears. Use this option when
    /// the final amount is not known when the checkout flow is initiated and you want to redirect the customer
    /// to the merchant page without processing the payment.
    Continue,
    /// After you redirect the customer to the PayPal payment page, a Pay Now button appears.
    /// Use this option when the final amount is known when the checkout is initiated and you want to
    /// process the payment immediately when the customer clicks Pay Now.
    PayNow,
}

impl Default for UserAction {
    fn default() -> Self {
        UserAction::Continue
    }
}

/// The merchant-preferred payment sources.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PayeePreferred {
    /// Accepts any type of payment from the customer.
    Unrestricted,
    /// Accepts only immediate payment from the customer.
    /// For example, credit card, PayPal balance, or instant ACH.
    /// Ensures that at the time of capture, the payment does not have the `pending` status.
    ImmediatePaymentRequired,
}

impl Default for PayeePreferred {
    fn default() -> Self {
        PayeePreferred::Unrestricted
    }
}

/// A payment method.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PaymentMethod {
    /// The customer-selected payment method on the merchant site.
    pub payer_selected: Option<String>,
    /// The merchant-preferred payment sources.
    pub payee_preferred: Option<PayeePreferred>,
}

/// Customize the payer experience during the approval process for the payment with PayPal.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ApplicationContext {
    /// The label that overrides the business name in the PayPal account on the PayPal site.
    pub brand_name: Option<String>,
    /// The BCP 47-formatted locale of pages that the PayPal payment experience shows. PayPal supports a five-character code.
    ///
    /// For example, da-DK, he-IL, id-ID, ja-JP, no-NO, pt-BR, ru-RU, sv-SE, th-TH, zh-CN, zh-HK, or zh-TW.
    pub locale: Option<String>,
    /// The type of landing page to show on the PayPal site for customer checkout
    pub landing_page: Option<LandingPage>,
    /// The shipping preference
    pub shipping_preference: Option<ShippingPreference>,
    /// Configures a Continue or Pay Now checkout flow.
    pub user_action: Option<UserAction>,
    /// The customer and merchant payment preferences.
    pub payment_method: Option<PaymentMethod>,
    /// The URL where the customer is redirected after the customer approves the payment.
    pub return_url: Option<String>,
    /// The URL where the customer is redirected after the customer cancels the payment.
    pub cancel_url: Option<String>,
}

/// A order payload to be used when creating an order.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct OrderPayload {
    /// The intent to either capture payment immediately or authorize a payment for an order after order creation.
    pub intent: Intent,
    /// The customer who approves and pays for the order. The customer is also known as the payer.
    pub payer: Option<Payer>,
    /// An array of purchase units. Each purchase unit establishes a contract between a payer and the payee.
    /// Each purchase unit represents either a full or partial order that the payer intends to purchase from the payee.
    pub purchase_units: Vec<PurchaseUnit>,
    /// Customize the payer experience during the approval process for the payment with PayPal.
    pub application_context: Option<ApplicationContext>,
}

impl OrderPayload {
    /// Creates a new order payload with the required properties.
    pub fn new<S: Into<Vec<PurchaseUnit>>>(intent: Intent, purchase_units: S) -> Self {
        Self {
            intent,
            purchase_units: purchase_units.into(),
            ..Default::default()
        }
    }
}

/// The card brand or network.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardBrand {
    /// Visa card.
    Visa,
    /// Mastecard card.
    Mastercard,
    /// Discover card.
    Discover,
    /// American Express card.
    Amex,
    /// Solo debit card.
    Solo,
    /// Japan Credit Bureau card.
    JCB,
    /// Military Star card.
    Star,
    /// Delta Airlines card.
    Delta,
    /// Switch credit card.
    Switch,
    /// Maestro credit card.
    Maestro,
    /// Carte Bancaire (CB) credit card.
    CbNationale,
    /// Configoga credit card.
    Configoga,
    /// Confidis credit card.
    Confidis,
    /// Visa Electron credit card.
    Electron,
    /// Cetelem credit card.
    Cetelem,
    /// China union pay credit card.
    ChinaUnionPay,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum CardType {
    Credit,
    Debit,
    Prepaid,
    Unknown,
}

/// The payment card to use to fund a payment.
#[derive(Debug, Serialize, Deserialize)]
pub struct CardResponse {
    /// The last digits of the payment card.
    pub last_digits: String,
    /// The card brand or network.
    pub brand: CardBrand,
    /// The payment card type.
    #[serde(rename = "type")]
    pub card_type: CardType,
}

/// The customer's wallet used to fund the transaction.
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletResponse {
    /// Apple Pay Wallet response information.
    pub apple_pay: CardResponse,
}

/// The payment source used to fund the payment.
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentSourceResponse {
    /// The payment card to use to fund a payment. Card can be a credit or debit card
    pub card: Option<CardResponse>,
    /// The customer's wallet used to fund the transaction.
    pub wallet: Option<WalletResponse>,
}

/// The status of an order.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    /// The order was created with the specified context.
    Created,
    /// The order was saved and persisted. The order status continues to be in progress until a capture
    /// is made with final_capture = true for all purchase units within the order.
    Saved,
    /// The customer approved the payment through the PayPal wallet or another form of guest or unbranded payment. For example, a card, bank account, or so on.
    Approved,
    /// All purchase units in the order are voided.
    Voided,
    /// The payment was authorized or the authorized payment was captured for the order.
    Completed,
}

/// An order represents a payment between two or more parties.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    /// The date and time when the transaction occurred.
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The date and time when the transaction was last updated.
    pub update_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The ID of the order.
    pub id: String,
    /// The payment source used to fund the payment.
    pub payment_source: Option<PaymentSourceResponse>,
    /// The intent to either capture payment immediately or authorize a payment for an order after order creation.
    pub intent: Option<Intent>,
    /// The customer who approves and pays for the order. The customer is also known as the payer.
    pub payer: Option<Payer>,
    /// An array of purchase units. Each purchase unit establishes a contract between a customer and merchant.
    /// Each purchase unit represents either a full or partial order that the customer intends to purchase from the merchant.
    pub purchase_units: Option<Vec<PurchaseUnit>>,
    /// The order status.
    pub status: OrderStatus,
    /// An array of request-related HATEOAS links. To complete payer approval, use the approve link to redirect the payer.
    pub links: Vec<LinkDescription>,
}

impl Client {
    /// Creates an order. Supports orders with only one purchase unit.
    pub async fn create_order(
        &mut self,
        order: OrderPayload,
        header_params: HeaderParams,
    ) -> Result<Order, ResponseError> {
        let builder = {
            self.setup_headers(
                self.client.post(&format!("{}/v2/checkout/orders", self.endpoint())),
                header_params,
            )
            .await
        };
        let res = builder.json(&order).send().await?;

        if res.status().is_success() {
            let order = res.json::<Order>().await?;
            Ok(order)
        } else {
            Err(ResponseError::ApiError(res.json::<PaypalError>().await?))
        }
    }

    /// Used internally for order requests that have no body.
    async fn build_endpoint_order(
        &mut self,
        order_id: &str,
        endpoint: &str,
        post: bool,
        header_params: crate::client::HeaderParams,
    ) -> Result<Order, ResponseError> {
        let format = format!("{}/v2/checkout/orders/{}/{}", self.endpoint(), order_id, endpoint);

        let builder = self
            .setup_headers(
                match post {
                    true => self.client.post(&format),
                    false => self.client.get(&format),
                },
                header_params,
            )
            .await;

        let res = builder.send().await?;

        if res.status().is_success() {
            let order = res.json::<Order>().await?;
            Ok(order)
        } else {
            Err(ResponseError::ApiError(res.json::<PaypalError>().await?))
        }
    }

    /// Updates an order with the CREATED or APPROVED status.
    /// You cannot update an order with the COMPLETED status.
    ///
    /// Only replacing the existing purchase units and intent is supported right now.
    ///
    /// Note: You can only update the intent from Authorize to Capture
    ///
    /// More info on what you can change: https://developer.paypal.com/docs/api/orders/v2/#orders_patch
    pub async fn update_order(
        &mut self,
        id: &str,
        intent: Option<Intent>,
        purchase_units: Option<Vec<PurchaseUnit>>,
    ) -> Result<(), ResponseError> {
        let mut intent_json = String::new();
        let units_json = String::new();

        if let Some(p_units) = purchase_units {
            let mut units_json = String::new();

            for (i, unit) in p_units.iter().enumerate() {
                let unit_str = serde_json::to_string(&unit).expect("error serializing purchase unit");
                let mut unit_json = format!(
                    r#"
                {{
                    "op": "replace",
                    "path": "/purchase_units/@reference_id='{reference_id}'",
                    "value": {unit}
                }}
                "#,
                    reference_id = unit.reference_id.clone().unwrap_or_else(|| String::from("default")),
                    unit = unit_str
                );

                if i < p_units.len() - 1 {
                    unit_json += ",";
                }

                units_json.push_str(&unit_json);
            }
        }

        if let Some(x) = intent {
            let intent_str = match x {
                Intent::Authorize => String::from("AUTHORIZE"),
                Intent::Capture => String::from("CAPTURE"),
            };

            intent_json = format!(
                r#"
                {{
                    "op": "replace",
                    "path": "/intent",
                    "value": "{intent}"
                }}
                "#,
                intent = intent_str
            );
        }

        let final_json = {
            if !intent_json.is_empty() && !units_json.is_empty() {
                format!("[{},{}]", intent_json, units_json)
            } else {
                format!("[{}{}]", intent_json, units_json)
            }
        };

        let builder = {
            self.setup_headers(
                self.client
                    .patch(&format!("{}/v2/checkout/orders/{}", self.endpoint(), id)),
                crate::client::HeaderParams {
                    content_type: Some(String::from("application/json")),
                    ..Default::default()
                },
            )
            .await
        };

        let res = builder.body(final_json.clone()).send().await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(ResponseError::ApiError(res.json::<PaypalError>().await?))
        }
    }

    /// Shows details for an order, by ID.
    pub async fn show_order_details(&mut self, order_id: &str) -> Result<Order, ResponseError> {
        self.build_endpoint_order(order_id, "", false, HeaderParams::default())
            .await
    }

    /// Captures payment for an order. To successfully capture payment for an order,
    /// the buyer must first approve the order or a valid payment_source must be provided in the request.
    /// A buyer can approve the order upon being redirected to the rel:approve URL that was returned in the HATEOAS links in the create order response.
    pub async fn capture_order(
        &mut self,
        order_id: &str,
        header_params: crate::client::HeaderParams,
    ) -> Result<Order, ResponseError> {
        self.build_endpoint_order(order_id, "capture", true, header_params)
            .await
    }

    /// Authorizes payment for an order. To successfully authorize payment for an order,
    /// the buyer must first approve the order or a valid payment_source must be provided in the request.
    /// A buyer can approve the order upon being redirected to the rel:approve URL that was returned in the HATEOAS links in the create order response.
    pub async fn authorize_order(
        &mut self,
        order_id: &str,
        header_params: HeaderParams,
    ) -> Result<Order, ResponseError> {
        self.build_endpoint_order(order_id, "authorize", true, header_params)
            .await
    }
}

// TODO: Add strong typed support for order errors in body: https://developer.paypal.com/docs/api/orders/v2/#errors
