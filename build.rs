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
}

fn build_for_release() {
    install_gschema();
    // copy_icons();
    install_desktop_file();
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

// fn copy_icons() {
//     let icons_path = "data/icons/";
//     // "~/.local/share/icons/hicolor/scalable/apps/",
//     // "~/.local/share/icons/hicolor/symbolic/apps/",

//     let status = Command::new("cp")
//         .args(&[
//             icons_path,
//             &format!("{HOME}/.local/share/icons/hicolor/scalable/apps/"),
//         ])
//         .status()
//         .expect("Should be able to copy icons");

//     println!("Copying icons: {status}");
// }

fn install_desktop_file() {
    let desktop_file = format!("data/{APP_ID}.desktop");
    let destination = format!("{HOME}/.local/share/applications/");
    create_directory_if_needed(&destination);

    copy(&desktop_file, &destination);
    update_desktop_databse(&destination);

    println!("cargo:rerun-if-changed={desktop_file}");
}

fn update_desktop_databse(location: &str) {
    let status = Command::new("update-desktop-databse")
        .arg(location)
        .status()
        .expect("Should be able to update desktop database");

    println!("Updating desktop database: {status}");
}
