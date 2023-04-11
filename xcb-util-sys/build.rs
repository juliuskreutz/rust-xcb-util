use std::{env, path::PathBuf};

fn main() {
    let mut bindings = bindgen::Builder::default();

    if cfg!(feature = "ewmh") {
        println!("cargo:rustc-link-lib=xcb-ewmh");
        println!("cargo:rerun-if-changed=wrappers/ewmh.h");

        bindings = bindings
            .header("wrappers/ewmh.h")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .wrap_static_fns(true)
            .allowlist_function("xcb_ewmh_.*")
            .allowlist_type("xcb_ewmh_.*");
    }

    bindings
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
