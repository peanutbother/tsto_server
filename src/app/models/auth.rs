#[cfg(feature = "server")]
use axum_login::AuthUser;
use indexmap::IndexSet;
#[cfg(feature = "server")]
use password_auth::generate_hash;
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use sqlx::prelude::FromRow;
use std::collections::HashSet;

pub type UserId = i64;
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct User {
    pub id: UserId,
    pub username: String,
    #[serde(skip)]
    pub password: String,
    pub role: Role,
}

impl User {
    #[cfg(feature = "server")]
    pub fn new(username: String, password: String, role: Role) -> Self {
        Self {
            id: 0,
            username,
            password: generate_hash(password),
            role,
        }
    }
}

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("password", &"[redacted]")
            .field("role", &self.role)
            .finish()
    }
}

#[cfg(feature = "server")]
impl AuthUser for User {
    type Id = UserId;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

impl std::fmt::Debug for Credentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Credentials")
            .field("username", &self.username)
            .field("password", &"[redacted]")
            .finish()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "server",
    derive(sqlx::Type),
    sqlx(type_name = "permissions", rename_all = "lowercase")
)]
pub enum Role {
    User,
    Moderator,
    Operator,
    Owner,
}

impl Role {
    pub fn all() -> IndexSet<Role> {
        [Role::User, Role::Moderator, Role::Operator, Role::Owner]
            .into_iter()
            .collect()
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Role::User => "user",
            Role::Moderator => "moderator",
            Role::Operator => "operator",
            Role::Owner => "owner",
        })
    }
}

impl From<Role> for HashSet<Role> {
    fn from(value: Role) -> Self {
        match value {
            Role::User => vec![Role::User],
            Role::Moderator => vec![Role::User, Role::Moderator],
            Role::Operator => vec![Role::User, Role::Moderator, Role::Operator],
            Role::Owner => vec![Role::User, Role::Moderator, Role::Operator, Role::Owner],
        }
        .into_iter()
        .collect()
    }
}
