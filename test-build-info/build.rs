extern crate build_info;

use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    build_info::gen_build_info(out_dir.as_ref(), "build_info.rs");
}
