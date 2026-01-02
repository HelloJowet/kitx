use field_access::FieldAccess;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt::Debug;

/// Status Enum
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Default, Hash, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Status {
    #[default]
    Draft,
    Published,
    Archived,
}

// Article
#[derive(Debug, Serialize, Deserialize, Default, FromRow, FieldAccess, Clone, PartialEq, Hash)]
pub struct Article {
    pub id: i32,
    pub tenant_id: i32,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default)]
    pub views: i32,
    #[serde(default)]
    pub deleted: bool,
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(default)]
    pub status: Status,
}

impl Article {
    pub fn new(tenant_id: i32, title: &str, content: Option<String>) -> Self {
        Article {
            tenant_id,
            title: title.to_string(),
            content,
            created_at: Some(chrono::Local::now().naive_local()),
            status: Status::Draft,
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, FromRow, FieldAccess, Clone, PartialEq, Hash)]
pub struct ArticleTag {
    pub article_id: i32,
    pub share_seq: i32,
    pub tag: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[cfg(any(feature = "mysql", feature = "sqlite", feature = "postgres"))]
impl ArticleTag {
    pub fn new(tag: &str) -> Self {
        ArticleTag {
            tag: tag.to_string(),
            ..Default::default()
        }
    }
}

pub mod mysql {
    use super::*;

    // TODO: support Enums for MySQL
    #[derive(Debug, Serialize, Deserialize, Default, FromRow, FieldAccess, Clone, PartialEq, Hash)]
    pub struct Article {
        pub id: i32,
        pub tenant_id: i32,
        pub title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub content: Option<String>,
        #[serde(default)]
        pub views: i32,
        #[serde(default)]
        pub deleted: bool,
        pub created_at: Option<chrono::NaiveDateTime>,
    }

    impl Article {
        pub fn new(tenant_id: i32, title: &str, content: Option<String>) -> Self {
            Article {
                tenant_id,
                title: title.to_string(),
                content,
                created_at: Some(chrono::Local::now().naive_local()),
                ..Default::default()
            }
        }
    }
}
