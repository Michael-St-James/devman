// User domain model

#[derive(Debug, Clone)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
}

impl User {
    pub fn new(id: u64, username: String, email: String) -> Self {
        Self {
            id,
            username,
            email,
        }
    }

    pub fn display_name(&self) -> String {
        format!("{} ({})", self.username, self.email)
    }
}
