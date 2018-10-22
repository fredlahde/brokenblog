use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Clone)]
pub struct Post {
    pub id: i64,
    pub content_raw: String,
    pub content_parsed: String,
    //pub date: DateTime<Utc>, TODO: Fix serde for date field
}

impl Post {
    // TODO: Impl
    pub fn new() -> Post {
        Post {
            id: 0,
            content_raw: String::new(),
            content_parsed: String::new(),
        }
    }
}
