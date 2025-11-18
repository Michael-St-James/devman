// Journal and logging functionality
use common::domain::organization::Organization;
use common::domain::user::User;
use common::helper::validation;

uniffi::setup_scaffolding!();

pub struct Calculator;

/// Returns a basic greeting for testing UniFII integration
pub fn get_greeting() -> String {
    let test = example_common_usage();
    format!(" Hello from Rust Journal! /n {}", test)
}

/// Example function demonstrating usage of common crate modules
pub fn example_common_usage() -> String {
    // Create a user
    let user = User::new(1, "john_doe".to_string(), "john@example.com".to_string());

    // Validate the user's email
    let is_valid = validation::is_valid_email(&user.email);

    // Create an organization
    let org = Organization::new(
        1,
        "Acme Corp".to_string(),
        Some("A great organization".to_string()),
    );

    // Return a formatted string with the results
    format!(
        "User: {} | Email Valid: {} | Organization: {} | Has Description: {}",
        user.display_name(),
        is_valid,
        org.name,
        org.has_description()
    )
}
