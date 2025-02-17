use std::path::PathBuf;

fn main() {
    add_extension_module_link_args();
    let my_package_dir: PathBuf =
        [env!("CARGO_MANIFEST_DIR"), "lua"].iter().collect();
    expose_shared_library::expose_shared_library(my_package_dir).unwrap();
}

#[cfg(target_os = "macos")]
fn add_extension_module_link_args() {
    println!("cargo::rustc-link-arg=-undefined");
    println!("cargo::rustc-link-arg=dynamic_lookup");
}

#[cfg(not(target_os = "macos"))]
fn add_extension_module_link_args() {}
