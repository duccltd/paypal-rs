//! The PayPal REST APIs use webhooks for event notification. Webhooks are HTTP callbacks 
//! that receive notification messages for events. After you configure a webhook listener 
//! for your app, you can create a webhook, which subscribes the webhook listener for your app to events. 
//! The notifications namespace contains resource collections for webhooks.
//!
//! Reference: https://developer.paypal.com/api/webhooks/v1/

use crate::common::*;
use crate::client::HeaderParams;
use crate::errors::{PaypalError, ResponseError};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use crate::client::{Client};

/// The verification status
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VerificationStatus {
    /// Webhook signature verified
    Success,
    /// Webhook signature was a failure
    Failure
}

/// Verification represents the status of the webhook signature
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Verification {
    /// The status of the signature verification.
    pub verification_status: VerificationStatus
}

/// Verifies a webhook signature.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookVerificationPayload<T> {
    /// The ID of the HTTP transmission. Contained in the PAYPAL-TRANSMISSION-ID header of the notification message.
    pub transmission_id: String,
    /// The date and time of the HTTP transmission, in Internet date and time format. Appears in the PAYPAL-TRANSMISSION-TIME header of the notification message.
    pub transmission_time: String,
    /// The X.509 public key certificate. Download the certificate from this URL and use it to verify the signature. Extract this value from the PAYPAL-CERT-URL response header, which is received with the webhook notification.
    pub cert_url: String,
    /// The algorithm that PayPal uses to generate the signature and that you can use to verify the signature. Extract this value from the PAYPAL-AUTH-ALGO response header, which is received with the webhook notification.
    pub auth_algo: String,
    /// The PayPal-generated asymmetric signature. Appears in the PAYPAL-TRANSMISSION-SIG header of the notification message.
    pub transmission_sig: String,
    /// The ID of the webhook as configured in your Developer Portal account.
    pub webhook_id: String,
    /// A webhook event notification.
    pub webhook_event: T
}

/// Webhook callback
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Webhook<T> {
    /// The ID of the webhook.
    pub id: String,
    /// The creation type of the webhook.
    pub create_time: String,
    /// The event type of the webhook.
    pub event_type: String,
    /// The resource type of the webhook body.
    pub resource_type: String,
    /// The resource version from the api.
    pub resource_version: String,
    /// Webhook summary description.
    pub summary: String,
    /// The resource from resource type.
    pub resource: T,
    /// Maybe unix timestamp?
    pub zts: Option<i64>,
    /// Version of the event
    pub event_version: String,
    /// An array of request-related HATEOAS links
    pub links: Vec<LinkDescription>,
}

impl Client {
    /// Verify webhook signature
    pub async fn verify_signature<T: Serialize>(
        &mut self,
        signature: WebhookVerificationPayload<T>,
        _header_params: HeaderParams,
    ) -> Result<Verification, ResponseError> {
        let builder = {
            self.setup_headers(
                self.client.post(&format!("{}/v1/notifications/verify-webhook-signature", self.endpoint())),
                crate::client::HeaderParams {
                    content_type: Some(String::from("application/json")),
                    ..Default::default()
                },
            )
            .await
        };
        let res = builder.json(&signature).send().await?;

        if res.status().is_success() {
            let verification = res.json::<Verification>().await?;
            Ok(verification)
        } else {
            Err(ResponseError::ApiError(res.json::<PaypalError>().await?))
        }
    }
}