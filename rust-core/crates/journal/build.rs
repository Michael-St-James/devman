fn main() {
    uniffi::generate_scaffolding("src/journal_core.udl")
        .expect("failed to generate UniFFI scaffolding");
}
