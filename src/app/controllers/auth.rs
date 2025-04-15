use std::collections::HashSet;

use crate::{
    app::models::auth::{Credentials, Role, User, UserId},
    database::Database,
};
use axum::async_trait;
use axum_login::{AuthSession, AuthnBackend, AuthzBackend};
use password_auth::{generate_hash, verify_password, VerifyError};
use tokio::task;
use tracing::{debug, error, instrument};

pub type Session = AuthSession<AuthController>;

#[derive(Debug, thiserror::Error)]
pub enum AuthControllerError {
    #[error("Unauthorized access to protected route")]
    Unauthorized,
    #[error("Missing permissions")]
    MissingPermissions,
    #[error("Internal Server Error")]
    Tokio(#[from] tokio::task::JoinError),
    #[error("Failed to verify password: {0}")]
    VerifyError(VerifyError),
    #[error("Internal Server Error")]
    Database(#[from] sqlx::Error),
}

#[derive(Debug, Clone)]
pub struct AuthController {
    db: Database,
}

impl AuthController {
    #[instrument]
    pub async fn set_username(
        &self,
        id: super::super::models::auth::UserId,
        username: String,
    ) -> Result<bool, AuthControllerError> {
        const QUERY: &str = "UPDATE auth SET username = ? WHERE id = ?";

        Ok(sqlx::query(QUERY)
            .bind(username)
            .bind(id)
            .execute(&self.db)
            .await?
            .rows_affected()
            == 1)
    }

    #[instrument]
    pub async fn set_password(
        &self,
        id: super::super::models::auth::UserId,
        password: String,
    ) -> Result<bool, AuthControllerError> {
        const QUERY: &str = "UPDATE auth SET password = ? WHERE id = ?";

        Ok(sqlx::query(QUERY)
            .bind(generate_hash(password))
            .bind(id)
            .execute(&self.db)
            .await?
            .rows_affected()
            == 1)
    }

    #[instrument]
    pub async fn set_role(
        &self,
        id: super::super::models::auth::UserId,
        role: Role,
    ) -> Result<bool, AuthControllerError> {
        const QUERY: &str = "UPDATE auth SET role = ? WHERE id = ?";

        Ok(sqlx::query(QUERY)
            .bind(role)
            .bind(id)
            .execute(&self.db)
            .await?
            .rows_affected()
            == 1)
    }

    #[instrument]
    pub async fn create_user(&self, user: User) -> Result<UserId, AuthControllerError> {
        const QUERY: &str = r#"INSERT INTO auth (username, password, role)
        OUTPUT Inserted.id 
        VALUES(?, ?, ?)"#;

        Ok(sqlx::query_scalar(QUERY)
            .bind(user.username)
            .bind(user.password)
            .bind(user.role)
            .fetch_one(&self.db)
            .await?)
    }

    #[instrument]
    pub async fn delete_user(&self, id: UserId) -> Result<bool, AuthControllerError> {
        const QUERY: &str = "DELETE FROM auth WHERE id = ?";

        Ok(sqlx::query(QUERY)
            .bind(id)
            .execute(&self.db)
            .await?
            .rows_affected()
            == 1)
    }
}

impl Default for AuthController {
    fn default() -> Self {
        Self {
            db: crate::database::DATABASE
                .get()
                .expect("database is initialized")
                .clone(),
        }
    }
}

#[async_trait]
impl AuthnBackend for AuthController {
    type User = User;
    type Credentials = Credentials;
    type Error = AuthControllerError;

    #[instrument]
    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        const QUERY: &str = "SELECT * FROM auth WHERE username = ?";

        debug!("fetching user with credentials");
        let user: Option<Self::User> = sqlx::query_as(QUERY)
            .bind(creds.username)
            .fetch_optional(&self.db)
            .await
            .inspect_err(|e| {
                error!("{e}");
            })?;

        debug!("verifying credentials");
        task::spawn_blocking(|| {
            Ok(user.filter(|user| verify_password(creds.password, &user.password).is_ok()))
        })
        .await?
    }

    #[instrument]
    async fn get_user(
        &self,
        user_id: &axum_login::UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        const QUERY: &str = "SELECT * FROM auth WHERE id = ?";

        let user = sqlx::query_as(QUERY)
            .bind(user_id)
            .fetch_optional(&self.db)
            .await
            .map_err(|e| {
                tracing::error!("{e}");
                e
            })?;

        Ok(user)
    }
}

#[async_trait]
impl AuthzBackend for AuthController {
    type Permission = Role;

    /// Gets the permissions for the provided user.
    async fn get_user_permissions(
        &self,
        user: &Self::User,
    ) -> Result<HashSet<Self::Permission>, Self::Error> {
        Ok(user.role.clone().into())
    }

    // TODO implemt group permissions
    // /// Gets the group permissions for the provided user.
    // async fn get_group_permissions(
    //     &self,
    //     _user: &Self::User,
    // ) -> Result<HashSet<Self::Permission>, Self::Error> {
    //     Ok(HashSet::new())
    // }
}
