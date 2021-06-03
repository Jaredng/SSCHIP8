extern crate vcpkg;

use std::env;

fn main() {

    println!("cargo:rerun-if-changed=wrapper.h");

    // Windows builds depend on PDCurses, which is built by the pancurses crate.
    // Unix builds depend on ncurses, which we need to build ourselves.
    if(env::var("CARGO_CFG_UNIX").is_ok()) {
        vcpkg::Config::new()
            .emit_includes(true)
            .find_package("ncurses")
            .unwrap();
    }

    vcpkg::Config::new()
        .emit_includes(true)
        .find_package("sdl2")
        .unwrap();
}
