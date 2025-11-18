use camino::Utf8PathBuf;
use std::env;
use uniffi_bindgen::bindings::KotlinBindingGenerator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <udl_file> <library_path> <out_dir>", args[0]);
        std::process::exit(1);
    }

    let udl_file = &args[1];
    let library_path = &args[2];
    let out_dir = Utf8PathBuf::from(&args[3]);

    // Create output directory
    std::fs::create_dir_all(&out_dir)?;

    // Generate Kotlin bindings
    uniffi_bindgen::generate_bindings(
        &Utf8PathBuf::from(udl_file),
        None,
        KotlinBindingGenerator,
        Some(&out_dir),
        Some(&Utf8PathBuf::from(library_path)),
        None,
        false,
    )?;

    println!("✓ Generated Kotlin bindings in {}", out_dir);
    Ok(())
}
