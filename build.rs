// build.rs is a script that Cargo (Rust's build system and package manager)
// executes before compiling the crate. It's used for build-time tasks like
// compiling non-Rust code, generating code, or setting linker arguments.

// Import necessary modules from the standard library.
use std::env; // For accessing environment variables like OUT_DIR.
use std::path::PathBuf; // For working with file paths in a platform-agnostic way.
use std::process::Command; // For running external commands like the assembler.

fn main() {
    // Get the path to the output directory for this build.
    // Cargo sets the OUT_DIR environment variable to a directory inside `target`
    // where build scripts can place their output.
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // Assemble the `src/boot.S` assembly file.
    // This section invokes an external assembler (`aarch64-linux-gnu-as`)
    // to compile the assembly code into an object file (`boot.o`).
    let status = Command::new("aarch64-linux-gnu-as") // The assembler command.
        .args(&[ // Arguments passed to the assembler.
            "src/boot.S", // The input assembly file.
            "-o", // Option to specify the output file name.
            // The output object file will be placed in the `OUT_DIR`
            // (e.g., target/debug/build/<crate-name>-<hash>/out/boot.o).
            out_dir.join("boot.o").to_str().unwrap(),
        ])
        .status() // Execute the command and wait for it to complete.
        // If the assembler command itself fails to run (e.g., not found), panic.
        .expect("Failed to run assembler. Make sure aarch64-linux-gnu-as is installed.");
    
    // Check if the assembler command executed successfully.
    if !status.success() {
        // If assembly failed (e.g., syntax error in boot.S), panic and stop the build.
        panic!("Failed to assemble boot.S");
    }
    
    // Instruct Cargo to link the compiled object file (`boot.o`) with the Rust crate.
    // `println!` statements with the "cargo:" prefix are special instructions for Cargo.
    // `cargo:rustc-link-arg=<arg>` passes `<arg>` directly to the linker (`rustc` invokes it).
    // Here, we're telling the linker to include `boot.o` in the final executable.
    println!(
        "cargo:rustc-link-arg={}",
        out_dir.join("boot.o").display() // The path to the object file.
    );
    
    // Tell Cargo to re-run this build script if `src/boot.S` changes.
    // This ensures that if the assembly source is modified, it gets recompiled.
    println!("cargo:rerun-if-changed=src/boot.S");
    // Tell Cargo to re-run this build script if `linker.ld` (the linker script) changes.
    // This is important because linker scripts define how the final executable is laid out in memory,
    // and changes to it might require a full rebuild or relinking.
    println!("cargo:rerun-if-changed=linker.ld");
}