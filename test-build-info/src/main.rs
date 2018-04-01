
extern crate clap;

include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

fn main() {

    let _ = clap::App::new("Test build.rs program")
        .version(get_build_info_str())
        .get_matches();

    println!("build_info: {:?}", get_build_info());
}
