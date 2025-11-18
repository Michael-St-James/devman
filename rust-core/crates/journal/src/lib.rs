// Journal and logging functionality
use std::sync::Arc;

uniffi::setup_scaffolding!();

pub struct Calculator;

impl Calculator {
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
