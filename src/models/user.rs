use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// represent a user in the system
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub fname: String,  // matches database column name
    pub pass_hash: String,  // matches database column name
    pub created_at: DateTime<Utc>,
}

/// Data needed to create a new user
#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub fname: String,
    pub password: String,  // will be hashed before storing
}

/// Data needed to update a user
#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub fname: Option<String>,
    pub email: Option<String>,
}

impl User {
    /// Creates a new User instance (before inserting to DB)
    /// 
    /// # Arguments
    /// * `email` - User's email address
    /// * `fname` - User's first name
    /// * `password_hash` - Pre-hashed password
    pub fn new(email: String, fname: String, password_hash: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            email,
            fname,
            pass_hash: password_hash,
            created_at: Utc::now(),
        }
    }
}