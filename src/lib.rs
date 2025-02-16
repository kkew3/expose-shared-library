mod expose_shared_library;

#[cfg(not(any(feature = "lua", feature = "py")))]
compile_error!("At least one of the features `lua` or `py` must be enabled.");

pub use expose_shared_library::expose_shared_library;
