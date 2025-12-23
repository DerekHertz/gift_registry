use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppResult;
use crate::models::{Group, GroupMember};

pub struct GroupRepository {
    pool: PgPool,
}

impl GroupRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    /// Creates a new group
    pub async fn create(&self, name: String, creator_id: Uuid) -> AppResult<Group> {
        // 1. Create Group instance (generates id, invite_code, created_at)
        let group = Group::new(name, creator_id);
        
        // 2. Insert into groups table
        let created_group = sqlx::query_as!(
            Group,
            r#"
            INSERT INTO groups (id, name, invite_code, creator_id, created_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, invite_code, creator_id, created_at
            "#,
            group.id,
            group.name,
            group.invite_code,
            group.creator_id,
            group.created_at
        )
        .fetch_one(&self.pool)
        .await?;
        
        // 3. Add creator as first group member
        self.add_member(created_group.id, creator_id).await?;
        
        Ok(created_group)
    }
    
    /// Finds group by ID
    pub async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Group>> {
        let group = sqlx::query_as!(
            Group,
            r#"
            SELECT id, name, invite_code, creator_id, created_at
            FROM groups
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(group)
    }
    
    /// Finds group by invite code
    pub async fn find_by_invite_code(&self, invite_code: &str) -> AppResult<Option<Group>> {
        let group = sqlx::query_as!(
            Group,
            r#"
            SELECT id, name, invite_code, creator_id, created_at
            FROM groups
            WHERE invite_code = $1
            "#,
            invite_code
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(group)
    }
    
    /// Adds a user to a group
    pub async fn add_member(&self, group_id: Uuid, user_id: Uuid) -> AppResult<GroupMember> {
        // 1. Create GroupMember instance (generates id and joined_at)
        let member = GroupMember::new(group_id, user_id);
        
        // 2. Insert into group_members table
        // Note: The UNIQUE constraint will prevent duplicates
        let created_member = sqlx::query_as!(
            GroupMember,
            r#"
            INSERT INTO group_members (id, group_id, user_id, joined_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id, group_id, user_id, joined_at
            "#,
            member.id,
            member.group_id,
            member.user_id,
            member.joined_at
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            // Check for duplicate entry (PostgreSQL error code 23505)
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.code() == Some("23505") {
                    return crate::errors::AppError::DuplicateEntry(
                        format!("User {} is already a member of group {}", user_id, group_id)
                    );
                }
            }
            // Convert other errors to AppError
            crate::errors::AppError::DatabaseError(e)
        })?;
        
        Ok(created_member)
    }
    
    /// Gets all members of a group
    pub async fn get_members(&self, group_id: Uuid) -> AppResult<Vec<Uuid>> {
        // Return list of user_ids
        let members = sqlx::query_as!(
            GroupMember,
            r#"
            SELECT id, group_id, user_id, joined_at
            FROM group_members
            WHERE group_id = $1
            "#,
            group_id
        )
        .fetch_all(&self.pool)
        .await?;

        // Extract just the user_ids
        let user_ids: Vec<Uuid> = members.into_iter().map(|m| m.user_id).collect();
        
        Ok(user_ids)
    }
}