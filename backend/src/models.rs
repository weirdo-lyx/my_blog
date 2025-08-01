use serde::{Deserialize, Serialize};

/// Represents a blog post in the database.
#[derive(Serialize, Deserialize, Clone, Debug, sqlx::FromRow)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
}

/// The request payload for creating a new post.
#[derive(Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub content: String,
}
