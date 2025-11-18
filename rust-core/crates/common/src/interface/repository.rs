// Repository trait interface

use crate::domain::organization::Organization;
use crate::domain::user::User;

/// Generic repository interface for data access
pub trait Repository<T> {
    fn find_by_id(&self, id: u64) -> Option<T>;
    fn save(&mut self, item: T) -> Result<(), String>;
    fn delete(&mut self, id: u64) -> Result<(), String>;
}

/// User-specific repository operations
pub trait UserRepository: Repository<User> {
    fn find_by_email(&self, email: &str) -> Option<User>;
    fn find_by_username(&self, username: &str) -> Option<User>;
}

/// Organization-specific repository operations
pub trait OrganizationRepository: Repository<Organization> {
    fn find_by_name(&self, name: &str) -> Option<Organization>;
    fn list_all(&self) -> Vec<Organization>;
}
