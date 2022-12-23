//! Locates installed executable inside directories specified in the *PATH*
//! environment variable.
//!
//! The goals of this implementations are:
//!
//! * No dependencies except the standard library.
//! * No unsafe code, no syscalls, no calls to libc.
//! * Compatibility with [Miri](https://github.com/rust-lang/miri).
//!
//! # Example
//!
//! **whereismybin** can be used as a library. Example:
//!
//! ```rust,no_run
//! use std::process::Command;
//!
//! use whereismybin::whereismybin;
//!
//! let my_tool = whereismybin("my-tool").expect("Could not find my-tool");
//! let output = Command::new(my_tool)
//!     .arg("--some-arg")
//!     .output()
//!     .expect("Failed to execute process");
//! let content = output.stdout;
//! ```
//!
//! # Why not which-rs?
//!
//! The [which-rs](https://crates.io/crates/which) crate is great, but
//! unfortunately it doesn't run on [Miri](https://github.com/rust-lang/miri)
//! and depends on [libc](https://crates.io/crates/libc) crate. **whereismybin**
//! aims to provide more lightweight alternative.
use std::{
    env,
    path::{Path, PathBuf},
};

pub fn whereismybin<T: AsRef<Path>>(binary_name: T) -> Option<PathBuf> {
    match env::var_os("PATH") {
        Some(paths) => {
            for dir in env::split_paths(&paths) {
                let full_path = dir.join(&binary_name);
                if full_path.is_file() {
                    return Some(full_path);
                }
            }
            None
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whereismybin_whoami() {
        let res = whereismybin("whoami");
        assert_eq!(res, Some(PathBuf::from("/usr/bin/whoami")));
    }

    #[test]
    fn test_whereismybin_expect_none() {
        let res = whereismybin("this-binary-does-not-exist");
        assert_eq!(res, None);
    }
}
