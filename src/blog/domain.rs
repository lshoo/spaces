#![warn(clippy::too_many_arguments)]
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use spin_sdk::pg::{self, Decode};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Article {
    id: i64,
    title: String,
    content: String,
    author: String,
    coauthor: Option<String>,
    category: Option<String>,
    created_at: i64,
    available: bool,
    updated_at: i64,
}

impl Article {
    pub fn new(
        id: i64,
        title: String,
        content: String,
        author: String,
        coauthor: Option<String>,
        category: Option<String>,
        created_at: i64,
        available: bool,
        updated_at: i64,
    ) -> Self {
        Self {
            id,
            title,
            content,
            author,
            coauthor,
            category,
            created_at,
            available,
            updated_at,
        }
    }

    pub fn create(
        id: i64,
        title: String,
        content: String,
        author: String,
        coauthor: Option<String>,
        category: Option<String>,
        created_at: i64,
    ) -> Self {
        Self::new(
            id, title, content, author, coauthor, category, created_at, true, created_at,
        )
    }
}

impl TryFrom<&pg::Row> for Article {
    type Error = anyhow::Error;

    fn try_from(row: &pg::Row) -> Result<Self, Self::Error> {
        let id = i64::decode(&row[0])?;
        let title = String::decode(&row[1])?;
        let content = String::decode(&row[2])?;
        let author = String::decode(&row[3])?;
        let coauthor = Option::<String>::decode(&row[4])?;
        let category = Option::<String>::decode(&row[5])?;
        let created_at = i64::decode(&row[6])?;
        let available = bool::decode(&row[7])?;
        let updated_at = i64::decode(&row[8])?;

        Ok(Self {
            id,
            title,
            content,
            author,
            coauthor,
            category,
            created_at,
            available,
            updated_at,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateArticleRequest {
    pub title: String,
    pub content: String,
    pub author: String,
    pub coauthor: Option<String>,
    pub category: Option<String>,
}

impl CreateArticleRequest {
    pub fn build(&self, id: i64, created_at: i64) -> Article {
        Article::create(
            id,
            self.title.clone(),
            self.content.to_owned(),
            self.author.clone(),
            self.coauthor.clone(),
            self.category.clone(),
            created_at,
        )
    }
}

impl TryFrom<Option<Bytes>> for CreateArticleRequest {
    fn try_from(value: Option<Bytes>) -> Result<Self, Self::Error> {
        match value {
            Some(b) => serde_json::from_slice(&b).map_err(anyhow::Error::from),
            None => anyhow::bail!("No body"),
        }
    }

    type Error = anyhow::Error;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateArticleRequest {
    pub title: String,
    pub content: String,
    pub author: String,
    pub coauthor: Option<String>,
    pub category: Option<String>,
    pub created_at: i64,
    pub available: bool,
}

impl UpdateArticleRequest {
    pub fn build(&self, id: i64, updated_at: i64) -> Article {
        Article::new(
            id,
            self.title.clone(),
            self.content.to_owned(),
            self.author.clone(),
            self.coauthor.clone(),
            self.category.clone(),
            self.created_at,
            self.available,
            updated_at,
        )
    }
}

impl TryFrom<Option<Bytes>> for UpdateArticleRequest {
    fn try_from(value: Option<Bytes>) -> Result<Self, Self::Error> {
        match value {
            Some(b) => serde_json::from_slice(&b).map_err(anyhow::Error::from),
            None => anyhow::bail!("No body"),
        }
    }

    type Error = anyhow::Error;
}
