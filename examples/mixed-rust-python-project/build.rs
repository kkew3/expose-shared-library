use std::path::PathBuf;

fn main() {
    pyo3_build_config::add_extension_module_link_args();
    let my_package_dir: PathBuf =
        [env!("CARGO_MANIFEST_DIR"), "python"].iter().collect();
    expose_shared_library::expose_shared_library(my_package_dir).unwrap();
}
