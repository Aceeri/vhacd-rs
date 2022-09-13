use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/VHACD.cpp");

    cc::Build::new()
        .cpp(true)
        .file("src/VHACD.cpp")
        .define("ENABLE_VHACD_IMPLEMENTATION", "1")
        .compile("v-hacd");

    let bindings = bindgen::Builder::default()
        .header("src/VHACD.cpp")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
