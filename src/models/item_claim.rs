use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ItemClaim {
    pub id: Uuid,
    pub item_id: Uuid,
    pub claimed_by: Uuid,  // matches database column name
    pub purchased: bool,    // Rust primitive type is lowercase
    pub claimed_at: DateTime<Utc>,
}

impl ItemClaim {
    pub fn new(item_id: Uuid, claimed_by: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            item_id,
            claimed_by,
            purchased: false,  // defaults to false
            claimed_at: Utc::now(),
        }
    }
}