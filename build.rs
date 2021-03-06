extern crate bindgen;

use std::env;
use std::path::PathBuf;



fn main() {

    println!("cargo:rustc-link-search=./lib/PDCurses-3.9/wincon");
    println!("cargo:rustc-link-lib=static=pdcurses");
    println!("cargo:rustc-link-search=./lib/SDL2/Release");
    println!("cargo:rustc-link-lib=sdl2");

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default().header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings.write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write to bindings!");
}