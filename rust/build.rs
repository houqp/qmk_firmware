extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // bindgen will pass -D from BINDGEN_EXTRA_CLANG_ARGS to clang
    let cflags = std::env::var("BINDGEN_CFLAGS").expect("Missing CFLAGS environment variable");
    let extra_clang_args = cflags
        .split(" ")
        .filter(|s| s.starts_with("-D"))
        .collect::<Vec<&str>>()
        .join(" ");
    std::env::set_var("BINDGEN_EXTRA_CLANG_ARGS", extra_clang_args);

    let bindings = bindgen::builder()
        .header("../quantum/quantum_keycodes.h")
        .header("../tmk_core/common/keycode.h")
        // .rustified_enum("hid_keyboard_keypad_usage")
        // .rustified_enum("internal_special_keycodes")
        // .rustified_enum("mouse_keys")
        // .rustified_enum("quantum_keycodes")
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
