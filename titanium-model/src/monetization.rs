//! Monetization types for Discord's premium features.
//!
//! Includes entitlements, subscriptions, and SKUs.

use crate::Snowflake;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// An entitlement represents a user's access to a premium offering.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Entitlement {
    /// ID of the entitlement.
    pub id: Snowflake,

    /// ID of the SKU.
    pub sku_id: Snowflake,

    /// ID of the parent application.
    pub application_id: Snowflake,

    /// ID of the user that is granted access.
    #[serde(default)]
    pub user_id: Option<Snowflake>,

    /// Type of entitlement.
    #[serde(rename = "type")]
    pub entitlement_type: EntitlementType,

    /// Whether the entitlement was deleted.
    #[serde(default)]
    pub deleted: bool,

    /// Start date at which the entitlement is valid (ISO8601 timestamp).
    #[serde(default)]
    pub starts_at: Option<String>,

    /// Date at which the entitlement is no longer valid (ISO8601 timestamp).
    #[serde(default)]
    pub ends_at: Option<String>,

    /// ID of the guild that is granted access.
    #[serde(default)]
    pub guild_id: Option<Snowflake>,

    /// Whether the entitlement has been consumed.
    #[serde(default)]
    pub consumed: bool,
}

/// Type of entitlement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum EntitlementType {
    /// Entitlement was purchased by user.
    Purchase = 1,
    /// Entitlement for Discord Nitro subscription.
    PremiumSubscription = 2,
    /// Entitlement was gifted by developer.
    DeveloperGift = 3,
    /// Entitlement was purchased by a dev in application test mode.
    TestModePurchase = 4,
    /// Entitlement was granted when the SKU was free.
    FreePurchase = 5,
    /// Entitlement was gifted by another user.
    UserGift = 6,
    /// Entitlement was claimed by user for free as a Nitro subscriber.
    PremiumPurchase = 7,
    /// Entitlement was purchased as an app subscription.
    ApplicationSubscription = 8,
}

/// A subscription represents a user's recurring purchase.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Subscription {
    /// ID of the subscription.
    pub id: Snowflake,

    /// ID of the user who is subscribed.
    pub user_id: Snowflake,

    /// List of SKUs subscribed to.
    pub sku_ids: Vec<Snowflake>,

    /// List of entitlements granted for this subscription.
    pub entitlement_ids: Vec<Snowflake>,

    /// Start of the current subscription period (ISO8601 timestamp).
    pub current_period_start: String,

    /// End of the current subscription period (ISO8601 timestamp).
    pub current_period_end: String,

    /// Current status of the subscription.
    pub status: SubscriptionStatus,

    /// When the subscription was canceled (ISO8601 timestamp).
    #[serde(default)]
    pub canceled_at: Option<String>,

    /// ISO3166-1 alpha-2 country code of the payment source.
    #[serde(default)]
    pub country: Option<String>,
}

/// Status of a subscription.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SubscriptionStatus {
    /// Subscription is active and scheduled to renew.
    Active = 0,
    /// Subscription is active but will not renew.
    Ending = 1,
    /// Subscription is inactive and not being charged.
    Inactive = 2,
}

/// A SKU (Stock Keeping Unit) represents a premium offering.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Sku {
    /// ID of SKU.
    pub id: Snowflake,

    /// Type of SKU.
    #[serde(rename = "type")]
    pub sku_type: SkuType,

    /// ID of the parent application.
    pub application_id: Snowflake,

    /// Customer-facing name of the premium offering.
    pub name: String,

    /// System-generated URL slug based on the SKU's name.
    pub slug: String,

    /// SKU flags as a bitfield.
    #[serde(default)]
    pub flags: u64,
}

/// Type of SKU.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SkuType {
    /// Durable one-time purchase.
    Durable = 2,
    /// Consumable one-time purchase.
    Consumable = 3,
    /// Recurring subscription.
    Subscription = 5,
    /// System-generated SKU applied to a subscription.
    SubscriptionGroup = 6,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entitlement() {
        let json = r#"{
            "id": "123",
            "sku_id": "456",
            "application_id": "789",
            "type": 8,
            "deleted": false,
            "consumed": false
        }"#;

        let entitlement: Entitlement = crate::json::from_str(json).unwrap();
        assert_eq!(
            entitlement.entitlement_type,
            EntitlementType::ApplicationSubscription
        );
    }

    #[test]
    fn test_subscription() {
        let json = r#"{
            "id": "123",
            "user_id": "456",
            "sku_ids": ["789"],
            "entitlement_ids": ["321"],
            "current_period_start": "2024-01-01T00:00:00.000Z",
            "current_period_end": "2024-02-01T00:00:00.000Z",
            "status": 0
        }"#;

        let subscription: Subscription = crate::json::from_str(json).unwrap();
        assert_eq!(subscription.status, SubscriptionStatus::Active);
    }
}
