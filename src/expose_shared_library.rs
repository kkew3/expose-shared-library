//! For `py` feature,
//! https://pyo3.rs/v0.22.2/building-and-distribution#manual-builds for
//! details.

use std::path::{Path, PathBuf};
use std::{env, fs, io};

#[cfg(all(target_os = "windows", feature = "py"))]
fn get_shared_lib_paths() -> (String, String) {
    let pkg_name = env::var("CARGO_PKG_NAME").unwrap();
    let source = [&pkg_name, ".dll"].concat();
    let dest = [&pkg_name, ".pyd"].concat();
    (source, dest)
}

#[cfg(all(target_os = "windows", feature = "lua"))]
fn get_shared_lib_paths() -> (String, String) {
    let pkg_name = env::var("CARGO_PKG_NAME").unwrap();
    let source = [&pkg_name, ".dll"].concat();
    let dest = [&pkg_name, ".dll"].concat();
    (source, dest)
}

#[cfg(all(target_os = "macos", any(feature = "py", feature = "lua")))]
fn get_shared_lib_paths() -> (String, String) {
    let pkg_name = env::var("CARGO_PKG_NAME").unwrap();
    let source = ["lib", &pkg_name, ".dylib"].concat();
    let dest = [&pkg_name, ".so"].concat();
    (source, dest)
}

#[cfg(all(target_os = "linux", any(feature = "py", feature = "lua")))]
fn get_shared_lib_paths() -> (String, String) {
    let pkg_name = env::var("CARGO_PKG_NAME").unwrap();
    let source = ["lib", &pkg_name, ".so"].concat();
    let dest = [&pkg_name, ".so"].concat();
    (source, dest)
}

fn ensure_not_exists(path: &Path) -> Result<(), io::Error> {
    match fs::remove_file(path) {
        Ok(_) => Ok(()),
        Err(err) => match err.kind() {
            io::ErrorKind::NotFound => Ok(()),
            _ => Err(err),
        },
    }
}

fn create_link(dylib_src: &Path, so: &Path) -> Result<(), io::Error> {
    symlink::symlink_file(dylib_src, so)?;
    Ok(())
}

/// Expose the built target as a shared library. `output_dir` is the directory
/// to place the shared library. Return [`io::Error`] on error.
pub fn expose_shared_library(
    output_dir: impl AsRef<Path>,
) -> Result<(), io::Error> {
    let (source, dest) = get_shared_lib_paths();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let build_output_dir = out_dir.ancestors().nth(3).unwrap();
    let so: PathBuf = [output_dir.as_ref(), Path::new(&dest)].iter().collect();
    let dylib_src: PathBuf =
        [build_output_dir, Path::new(&source)].iter().collect();
    ensure_not_exists(&so).and_then(|_| create_link(&dylib_src, &so))
}
