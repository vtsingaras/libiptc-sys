extern crate pkg_config;

use std::env;

fn main() {
    if cfg!(target_os = "linux") {
        if let Ok(pc) = pkg_config::find_library("libiptc") {
            let paths = env::join_paths(pc.include_paths).unwrap();
            println!("cargo:include={}", paths.to_str().unwrap());
            //Check if pkg_config result has ip4tc defined
            //TODO: see why it doesn't work on some systems
            let mut ip4tc_found: bool = false;
            for library in pc.libs {
                if library.contains("ip4tc") {
                    ip4tc_found = true;
                }
            }
            //Hardcode link libs if not found
            //requires that library path was found correctly
            if !ip4tc_found {
                println!("cargo:rustc-link-lib=ip4tc");
                println!("cargo:rustc-link-lib=ip4tc");
            }
        } else {
            panic!("Please install iptables-devel or equivalent.");
        }
    } else {
        panic!("Only Linux is supported!");
    }
}
