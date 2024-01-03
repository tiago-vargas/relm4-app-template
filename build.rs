use std::{env, process::Command};

const APP_ID: &str = "com.github.tiago_vargas.relm4_app_template";
const HOME: &str = env!("HOME");

fn main() {
    install_gschema();
}

fn install_gschema() {
    let gschema_file = format!("data/{APP_ID}.gschema.xml");
    let destination = format!("{HOME}/.local/share/glib-2.0/schemas/");
    create_directory_if_needed(&destination);

    copy(&gschema_file, &destination);
    compile_schemas(&destination);

    println!("cargo:rerun-if-changed={gschema_file}");
}

fn create_directory_if_needed(name: &str) {
    let status = Command::new("mkdir")
        .args(&[
            name,
            "--parents",
        ])
        .status()
        .expect("Should be able to create directory");

    println!("Creating directory: {status}");
}

fn copy(source: &str, destination: &str) {
    let status = Command::new("cp")
        .args(&[
            source,
            destination,
        ])
        .status()
        .expect("Should be able to copy file");

    println!("Copying file: {status}");
}

fn compile_schemas(location: &str) {
    let status = Command::new("glib-compile-schemas")
        .arg(location)
        .status()
        .expect("Should be able to compile schemas");

    println!("Compiling schemas: {status}");
}
