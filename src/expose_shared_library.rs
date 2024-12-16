//! See https://pyo3.rs/v0.22.2/building-and-distribution#manual-builds for
//! details.

use std::path::{Path, PathBuf};
use std::{env, fs, io};

#[cfg(target_os = "windows")]
fn get_shared_lib_paths() -> (String, String) {
    let pkg_name = env::var("CARGO_PKG_NAME").unwrap();
    // From my experiment, the output dll name does not start with "lib", at
    // least on Windows 10.
    let source: String = [&pkg_name, ".dll"].concat();
    let dest: String = [&pkg_name, ".pyd"].concat();
    (source, dest)
}

#[cfg(target_os = "macos")]
fn get_shared_lib_paths() -> (String, String) {
    let pkg_name = env::var("CARGO_PKG_NAME").unwrap();
    let source: String = ["lib", &pkg_name, ".dylib"].concat();
    let dest: String = [&pkg_name, ".so"].concat();
    (source, dest)
}

#[cfg(target_os = "linux")]
fn get_shared_lib_paths() -> (String, String) {
    let pkg_name = env::var("CARGO_PKG_NAME").unwrap();
    let source: String = ["lib", &pkg_name, ".so"].iter().collect();
    let dest: String = [&pkg_name, ".so"].iter().collect();
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

/// Expose the built target as a shared library for python. `pylib_dir` is the
/// directory to place the shared library. Return [`io::Error`] on error.
pub fn expose_shared_library(
    pylib_dir: impl AsRef<Path>,
) -> Result<(), io::Error> {
    let (source, dest) = get_shared_lib_paths();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let so: PathBuf = [pylib_dir.as_ref(), Path::new(&dest)].iter().collect();
    let profile = env::var("PROFILE").unwrap();
    let dylib_src: PathBuf = [
        Path::new(&manifest_dir),
        Path::new("target"),
        Path::new(&profile),
        Path::new(&source),
    ]
    .iter()
    .collect();
    ensure_not_exists(&so).and_then(|_| create_link(&dylib_src, &so))
}
