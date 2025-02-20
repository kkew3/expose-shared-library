cfg_if::cfg_if! {
    if #[cfg(all(feature = "py", not(any(feature = "lua"))))] {
        fn main() {}
    } else if #[cfg(all(feature = "lua", not(any(feature = "py"))))] {
        fn main() {}
    } else {
        fn main() {
            compile_error!("You can enable one of the features: py, lua");
        }
    }
}
