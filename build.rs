extern crate bindgen;
extern crate vcpkg;

use std::env;
use std::path::PathBuf;
use std::collections::HashSet;

#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

// A workaround for https://github.com/rust-lang/rust-bindgen/issues/687
// This is necessary in order to bind anything that includes glibc's version of math.h
impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

fn main() {

    let ignored_macros = IgnoreMacros(
        vec![
            "FP_INFINITE".into(),
            "FP_NAN".into(),
            "FP_NORMAL".into(),
            "FP_SUBNORMAL".into(),
            "FP_ZERO".into(),
            "IPPORT_RESERVED".into(),   
        ]
        .into_iter()
        .collect(),
    );

    println!("cargo:rerun-if-changed=wrapper.h");

    vcpkg::Config::new()
        .emit_includes(true)
        .find_package("ncurses")
        .unwrap();

    vcpkg::Config::new()
        .emit_includes(true)
        .find_package("sdl2")
        .unwrap();

    let bindings = bindgen::Builder::default().header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .parse_callbacks(Box::new(ignored_macros))
        .rustfmt_bindings(true)
        .clang_arg("-Itarget/vcpkg/installed/x64-linux/include/")
        .generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings.write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write to bindings!");

}
