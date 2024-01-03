use std::{env, process::Command};

const APP_ID: &str = "com.github.tiago_vargas.relm4_app_template";
const HOME: &str = env!("HOME");

fn main() {
    let profile = env::var("PROFILE").unwrap();
    match profile.as_str() {
        "debug" => build_for_debug(),
        "release" => build_for_release(),
        _ => unreachable!(),
    }
}

fn build_for_debug() {
    install_gschema();
    install_icons();
}

fn build_for_release() {
    install_gschema();
    install_icons();
    install_desktop_file();
}

fn install_gschema() {
    let gschema_file = format!("data/{APP_ID}.gschema.xml");

    let is_valid = validate_gschema("data/");
    if is_valid {
        let destination = format!("{HOME}/.local/share/glib-2.0/schemas/");
        create_directory_if_needed(&destination);
        copy(&gschema_file, &destination);
        compile_schemas(&destination);
    }

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

fn validate_gschema(location: &str) -> bool {
    let status = Command::new("glib-compile-schemas")
        .args(&[
            location,
            "--strict",
            "--dry-run",
        ])
        .status()
        .expect("Should be able to validate schemas");

    println!("Validating schemas: {status}");

    status.success()
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

fn install_icons() {
    let icons = format!("{HOME}/data/icons/hicolor");
    let destination = format!("{HOME}/.local/share/icons/hicolor/");
    copy_icons_and_create_path_if_needed(&icons, &destination);
    update_icon_cache(&destination);
}

fn copy_icons_and_create_path_if_needed(source: &str, destination: &str) {
    let status = Command::new("rsync")
        .args(&[
            source,
            destination,
            "--archive",
        ])
        .status()
        .expect("Should be able to copy icons");

    println!("Copying icons: {status}");
}

fn update_icon_cache(location: &str) {
    let status = Command::new("gtk-update-icon-cache")
        .args(&[
            location,
            "--ignore-theme-index",
        ])
        .status()
        .expect("Should be able to update icon cache");

    println!("Updating icon cache: {status}");
}

fn install_desktop_file() {
    let desktop_file = format!("data/{APP_ID}.desktop");

    let is_valid = validate_desktop_file(&desktop_file);
    if is_valid {
        let destination = format!("{HOME}/.local/share/applications/");
        create_directory_if_needed(&destination);
        copy(&desktop_file, &destination);
        update_desktop_databse(&destination);
    }

    println!("cargo:rerun-if-changed={desktop_file}");
}

fn validate_desktop_file(file: &str) -> bool {
    let status = Command::new("desktop-file-validate")
        .arg(file)
        .status()
        .expect("Should be able to validate desktop file");

    println!("Validating desktop file: {status}");

    status.success()
}

fn update_desktop_databse(location: &str) {
    let status = Command::new("update-desktop-databse")
        .arg(location)
        .status()
        .expect("Should be able to update desktop database");

    println!("Updating desktop database: {status}");
}
