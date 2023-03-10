extern crate bindgen;

use std::{
    env,
    env::var,
    fs::{canonicalize, File},
    io::{copy, Write},
    path::{Path, PathBuf},
    process::Command,
};

fn run(command: &mut Command) {
    println!("Running: {:?}", command);
    match command.status() {
        Ok(status) => {
            if !status.success() {
                panic!("`{:?}` failed: {}", command, status);
            }
        }
        Err(error) => {
            panic!("failed to execute `{:?}`: {}", command, error);
        }
    }
}

fn build(out_dir: &Path) {
    let src_dir = PathBuf::from(var("CARGO_MANIFEST_DIR").unwrap()).join("necpp");
    let out_src_dir = out_dir.join("src");
    fs_extra::dir::copy(
        src_dir,
        &out_src_dir,
        &fs_extra::dir::CopyOptions {
            overwrite: true,
            skip_exist: false,
            buffer_size: 64000,
            copy_inside: true,
            depth: 0,
            content_only: false,
        },
    )
    .unwrap();

    if !out_dir.join("lib/libnecpp.a").exists() {
        build_necpp(&[], &out_src_dir, &out_dir);
    }
}

fn build_necpp(_flags: &[&str], src_dir: &Path, out_dir: &Path) {
    run(Command::new("make")
        .arg("-f")
        .arg("Makefile.git")
        .current_dir(&src_dir));

    run(
        Command::new(canonicalize(src_dir.join("configure")).unwrap())
            .arg("--without-lapack")
            .arg(format!("--prefix={}", out_dir.display()))
            .current_dir(&src_dir),
    );
    run(Command::new("make")
        .arg(format!("-j{}", var("NUM_JOBS").unwrap()))
        .current_dir(&src_dir));
    run(Command::new("make").arg("install").current_dir(&src_dir));
}

fn main() {

    let out_dir = PathBuf::from(var("OUT_DIR").unwrap());

    build(&out_dir);

    println!("cargo:rustc-link-search={}", out_dir.join("lib").display());
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=necpp");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    let header=PathBuf::from(var("OUT_DIR").unwrap()).join("include/libnecpp.h");
    format!("cargo:rerun-if-changed={:?}", header);
    //println!(format!("cargo:rerun-if-changed={}",header ));

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(header.to_str().expect("invalid path"))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
