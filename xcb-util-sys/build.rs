use std::{env, path::PathBuf};

fn main() {
    env::set_current_dir("wrappers").unwrap();

    let bindings = bindgen::Builder::default()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .prepend_enum_name(false)
        .blocklist_type("xcb_connection_t");

    let output_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    if cfg!(feature = "cursor") {
        println!("cargo:rustc-link-lib=xcb-cursor");
        println!("cargo:rerun-if-changed=cursor.h");

        bindings
            .clone()
            .header("cursor.h")
            .allowlist_type("xcb_cursor_.*")
            .allowlist_function("xcb_cursor_.*")
            .allowlist_var("xcb_cursor_.*")
            .generate()
            .expect("Unable to generate bindings")
            .write_to_file(output_path.join("cursor.rs"))
            .expect("Couldn't write bindings!");
    }

    if cfg!(feature = "ewmh") {
        println!("cargo:rustc-link-lib=xcb-ewmh");
        println!("cargo:rerun-if-changed=ewmh.h");

        let bindings = bindings
            .clone()
            .wrap_static_fns(true)
            .header("ewmh.h")
            .allowlist_type("xcb_ewmh_.*")
            .allowlist_function("xcb_ewmh_.*")
            .allowlist_var("xcb_ewmh_.*")
            .generate()
            .expect("Unable to generate bindings");

        cc::Build::new()
            .file(std::env::temp_dir().join("bindgen").join("extern.c"))
            .include(".")
            .compile("extern_ewmh");

        bindings
            .write_to_file(output_path.join("ewmh.rs"))
            .expect("Couldn't write bindings!");
    }

    if cfg!(feature = "icccm") {
        println!("cargo:rustc-link-lib=xcb-icccm");
        println!("cargo:rerun-if-changed=icccm.h");

        let bindings = bindings
            .clone()
            .header("icccm.h")
            .allowlist_type("xcb_icccm_.*")
            .allowlist_function("xcb_icccm_.*")
            .generate()
            .expect("Unable to generate bindings");

        bindings
            .write_to_file(output_path.join("icccm.rs"))
            .expect("Couldn't write bindings!");
    }
}
