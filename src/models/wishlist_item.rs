use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WishlistItem {
    pub id: Uuid,
    pub group_id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub price: Option<Decimal>,
    pub url: Option<String>,
    pub image_url: Option<String>,
    pub priority: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateWishlistItem {
    pub title: String,
    pub description: Option<String>,
    pub price: Option<Decimal>,
    pub url: Option<String>,
    pub image_url: Option<String>,
    pub priority: Option<String>,
    // Note: group_id and user_id will come from authenticated user context
}

#[derive(Debug, Deserialize)]
pub struct UpdateWishlistItem {
    pub title: Option<String>,
    pub description: Option<String>,
    pub price: Option<Decimal>,
    pub url: Option<String>,
    pub image_url: Option<String>,
    pub priority: Option<String>,
}

impl WishlistItem {
    pub fn new(
        group_id: Uuid,
        user_id: Uuid,
        title: String,
        description: Option<String>,
        price: Option<Decimal>,
        url: Option<String>,
        image_url: Option<String>,
        priority: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            group_id,
            user_id,
            title,
            description,
            price,
            url,
            image_url,
            priority,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}