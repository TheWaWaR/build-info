extern crate clap;

include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

fn main() {
    let _ = clap::App::new("Test build.rs program")
        .version(get_build_info_str(true))
        .long_version(get_build_info_str(false))
        .get_matches();

    println!("get_build_info_str(true): {}", get_build_info_str(true));
    println!("get_build_info_str(false): {}", get_build_info_str(false));

    let info = get_build_info();
    println!("build_info: {:?}", info);
    println!("logo:\n{}", info.0)
}
