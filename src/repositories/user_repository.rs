use sqlx::PgPool;
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST, BcryptResult};

use crate::errors::AppResult;
use crate::models::{User, CreateUser};

/// Repository for User database operations
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    /// Creates a new UserRepository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    /// Creates a new user in the database
    /// 
    /// # Arguments
    /// * `create_user` - User creation data
    /// 
    /// # Returns
    /// * Created User with generated ID
    pub async fn create(&self, create_user: CreateUser) -> AppResult<User> {
        // 1. Hash the password using bcrypt
        let hashed_pass = hash(create_user.password, DEFAULT_COST)
            .map_err(|e| crate::errors::AppError::ValidationError(
                format!("Password hashing failed: {}", e)
            ))?;
        
        // 2. Create User struct with new() - generates id and created_at
        let user = User::new(
            create_user.email,
            create_user.fname,
            hashed_pass,
        );
        
        // 3. Use sqlx::query_as! to INSERT and return User directly
        let created_user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, email, fname, pass_hash, created_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, email, fname, pass_hash, created_at
            "#,
            user.id,
            user.email,
            user.fname,
            user.pass_hash,
            user.created_at
        )
        .fetch_one(&self.pool)
        .await?;
        
        // 4. Return the created user
        Ok(created_user)
    }
    
    /// Finds a user by ID
    pub async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>> {
        // Use sqlx::query_as! to SELECT
        // Return None if not found
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, fname, pass_hash, created_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(user)
    }
    
    /// Finds a user by email
    pub async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, fname, pass_hash, created_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }
    
    /// Lists all users (useful for debugging)
    pub async fn list_all(&self) -> AppResult<Vec<User>> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, fname, pass_hash, created_at
            FROM users
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }
    
    /// Deletes a user by ID
    pub async fn delete(&self, id: Uuid) -> AppResult<bool> {
        // DELETE WHERE id = $1
        // Return true if deleted, false if not found
        let result = sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        // Check if any rows were affected
        Ok(result.rows_affected() > 0)
    }
}