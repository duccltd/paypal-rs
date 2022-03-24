//! A capture

use crate::{common::*, orders::*};
use serde::{Deserialize, Serialize};

/// Seller protection status
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(non_camel_case_types)]
pub enum SellerProtectionStatus {
    /// Seller protection eligability
    Eligable,
}

/// Seller protection
#[derive(Debug, Serialize, Deserialize)]
pub struct SellerProtection {
    /// Dispute categories 
    pub dispute_categories: Vec<String>,
    /// Status
    pub status: SellerProtectionStatus,
}

/// Related identifiers
#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedIds {
    /// Order ID
    pub order_id: String,
}

/// Supplementary data
#[derive(Debug, Serialize, Deserialize)]
pub struct SupplementaryData {
    /// Related identifiers
    pub related_ids: RelatedIds,
}

/// Seller receivable breakdown
#[derive(Debug, Serialize, Deserialize)]
pub struct SellerReceivableBreakdown {
    /// Paypal fee
    pub paypal_fee: Amount,
    /// Gross amount
    pub gross_amount: Amount,
    /// Net amount
    pub net_amount: Amount
}

/// Payment
#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    /// Payment amount
    pub amount: Amount,
    /// Seller protection 
    pub seller_protection: Option<SellerProtection>,
    /// The date and time when the payment occurred.
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The date and time when the payment was last updated.
    pub update_time: Option<chrono::DateTime<chrono::Utc>>,
    /// Final capture level
    pub final_capture: Option<bool>,
    /// Seller receivable breakdown
    pub seller_receivable_breakdown: Option<SellerReceivableBreakdown>,
    /// Custom identifier
    pub custom_id: Option<String>,
    /// An array of request-related HATEOAS links. To complete payer approval, use the approve link to redirect the payer.
    pub links: Vec<LinkDescription>,
    /// Capture identifier
    pub id: Option<String>,
    /// Capture status
    pub status: Option<CaptureStatus>,
}
