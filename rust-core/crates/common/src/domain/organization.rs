// Organization domain model

#[derive(Debug, Clone)]
pub struct Organization {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
}

impl Organization {
    pub fn new(id: u64, name: String, description: Option<String>) -> Self {
        Self {
            id,
            name,
            description,
        }
    }

    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }
}
