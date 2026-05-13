#[derive(Debug)]
#[derive(Clone)]
pub struct Document {
    pub title: String,
    content: String,
    pub author: String,
}

impl Document {
    pub fn new(title: &str, content: &str, author: &str) -> Self {
        Self {
            title: title.to_string(),
            content: content.to_string(),
            author: author.to_string(),
        }
    }
}
