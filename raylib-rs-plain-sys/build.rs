extern crate bindgen;

use std::fs::ReadDir;
use std::fs;
use std::path::PathBuf;

const RAYLIB_REPOSITORY_PATH: &str = "./native-src/raylib";
const RAYLIB_VERSION: &str = "4.2.0";

fn main() {
    clone_raylib();
    build_raylib();

     let bindings = bindgen::Builder::default()
        .header(RAYLIB_REPOSITORY_PATH.to_string() + "/src/raylib.h")
        .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: false })
        .raw_line(r"#![allow(non_upper_case_globals)]")
        .raw_line(r"#![allow(non_camel_case_types)]")
        .raw_line(r"#![allow(non_snake_case)]")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("src");

        bindings
        .write_to_file(out_path.join("lib.rs"))
        .expect("Couldn't write bindings!");

}

fn clone_raylib() {
    // TODO: Don't clone if the folder exists.
    // - No need to pull. Because it is a revision specification, it never changes.
    std::process::Command::new("git")
        .arg("clone")
        .arg("--depth=1")
        .arg("--single-branch")
        .arg("--branch")
        .arg(RAYLIB_VERSION)
        .arg("https://github.com/raysan5/raylib.git")
        .arg(RAYLIB_REPOSITORY_PATH)
        .status()
        .expect("Cannot clone raylib repository");
}

fn build_raylib() {
    let mut build: cc::Build = cc::Build::new();
    // Hide warnings
    build.warnings(false)
        .flag("-Wno-implicit-function-declaration")
    // Add defines
        .define("UNICODE", None)
        .define("PLATFORM_DESKTOP", None);
    
    // List c in the folder and make it a compilation target
    let entries: ReadDir = fs::read_dir(RAYLIB_REPOSITORY_PATH.to_string() + "/src/").unwrap();
    for entry in entries {
        let entry_path: PathBuf = entry.unwrap().path();
        let file_path = entry_path.to_string_lossy();
        if let Some(extension) = entry_path.extension() {
            // Add only the extension c
            if extension == "c" {
                println!("Add '{}' to compile target", file_path);
                build.file(file_path.to_string());
            }
        }
        
    }

    build
    // Add includes
        .include(RAYLIB_REPOSITORY_PATH.to_string() + "/src")
        .include(RAYLIB_REPOSITORY_PATH.to_string() + "/src/external/glfw/include")

        .compile("raylib");

    // Add native libs
    println!("cargo:rustc-link-lib=winmm");
    println!("cargo:rustc-link-lib=gdi32");
    println!("cargo:rustc-link-lib=shell32");
}