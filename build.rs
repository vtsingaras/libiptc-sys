extern crate pkg_config;

use std::env;

fn main() {
    if cfg!(target_os = "linux") {
        if let Ok(pc) = pkg_config::find_library("libiptc") {
            let paths = env::join_paths(pc.include_paths).unwrap();
            println!("cargo:include={}", paths.to_str().unwrap());
        } else {
            panic!("Please install iptables-devel or equivalent.");
        }
    } else {
        panic!("Only Linux is supported!");
    }
}