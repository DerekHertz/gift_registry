use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GroupMember {
    pub id: Uuid,
    pub group_id: Uuid,
    pub user_id: Uuid,
    pub joined_at: DateTime<Utc>
}

impl GroupMember {
    pub fn new(group_id: Uuid, user_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            group_id,
            user_id,
            joined_at: Utc::now(),
        }
    }
}