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
    ) -> Self {
        Self {
            id,
            title,
            content,
            author,
            coauthor,
            category,
            created_at,
        }
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

        Ok(Self::new(
            id, title, content, author, coauthor, category, created_at,
        ))
    }
}
