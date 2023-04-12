use std::{env, path::PathBuf};

fn main() {
    let bindings = bindgen::Builder::default()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .wrap_static_fns(true)
        .prepend_enum_name(false)
        .blocklist_type("xcb_connection_t");

    if cfg!(feature = "cursor") {
        println!("cargo:rustc-link-lib=xcb-cursor");
        println!("cargo:rerun-if-changed=wrappers/cursor.h");

        bindings
            .clone()
            .header("wrappers/cursor.h")
            .allowlist_type("xcb_cursor_.*")
            .allowlist_function("xcb_cursor_.*")
            .allowlist_var("xcb_cursor_.*")
            .generate()
            .expect("Unable to generate bindings")
            .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("cursor.rs"))
            .expect("Couldn't write bindings!");
    }

    if cfg!(feature = "ewmh") {
        println!("cargo:rustc-link-lib=xcb-ewmh");
        println!("cargo:rerun-if-changed=wrappers/ewmh.h");

        bindings
            .clone()
            .header("wrappers/ewmh.h")
            .allowlist_type("xcb_ewmh_.*")
            .allowlist_function("xcb_ewmh_.*")
            .allowlist_var("xcb_ewmh_.*")
            .generate()
            .expect("Unable to generate bindings")
            .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("ewmh.rs"))
            .expect("Couldn't write bindings!");
    }

    if cfg!(feature = "icccm") {
        println!("cargo:rustc-link-lib=xcb-icccm");
        println!("cargo:rerun-if-changed=wrappers/icccm.h");

        bindings
            .clone()
            .header("wrappers/icccm.h")
            .allowlist_type("xcb_icccm_.*")
            .allowlist_function("xcb_icccm_.*")
            .generate()
            .expect("Unable to generate bindings")
            .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("icccm.rs"))
            .expect("Couldn't write bindings!");
    }
}
