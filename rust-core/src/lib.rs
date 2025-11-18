use std::sync::Arc;

#[derive(uniffi::Object)]
pub struct Calculator;

#[uniffi::export]
impl Calculator {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Calculator)
    }

    pub fn add(&self, a: u32, b: u32) -> u32 {
        a + b
    }

    pub fn greet(&self, name: String) -> String {
        format!("Hello, {name}, from Rust!")
    }
}

// This line pulls in the UniFFI-generated glue at compile time.
// "my_core" must match the UDL file stem (my_core.udl).
uniffi::setup_scaffolding!("my_core");