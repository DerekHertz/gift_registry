use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use rand::{Rng, thread_rng};
use rand::distr::Alphanumeric;

/// Represents a gift exchange group
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub invite_code: String,
    pub creator_id: Uuid,
    pub created_at: DateTime<Utc>
}

/// Data needed to create a new group
#[derive(Debug, Deserialize)]
pub struct CreateGroup {
    pub name: String,
    // Creator will come from authenticated user context
}

impl Group {
    /// Creates a new Group instance
    /// 
    /// # Arguments
    /// * `name` - Group name
    /// * `creator_id` - UUID of the user creating the group
    pub fn new(name: String, creator_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            invite_code: Self::generate_invite_code(),
            creator_id,
            created_at: Utc::now(),
        }
    }
    
    /// Generates a random 8-character alphanumeric invite code
    /// Example: "AB12CD34"
    fn generate_invite_code() -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect()
    }
}