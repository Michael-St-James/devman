fn main() {
    uniffi::generate_scaffolding("src/auth_core.udl")
        .expect("failed to generate UniFFI scaffolding");
}
