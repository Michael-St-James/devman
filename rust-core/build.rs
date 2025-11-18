fn main() {
    // Re-run the build script if the UDL file changes
    println!("cargo:rerun-if-changed=src/my_core.udl");

    // Generate the UniFFI scaffolding from the UDL file
    uniffi_build::generate_scaffolding("src/my_core.udl")
        .expect("failed to generate UniFFI scaffolding");
}
