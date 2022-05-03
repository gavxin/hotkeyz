//! # hotkeyz
//! A auto keyboard and mouse tool for windows os.

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate error_chain;
mod errors {
    error_chain! {
      foreign_links {
        Io(::std::io::Error);
        WinOs(::windows::core::Error) #[cfg(windows)];
      }
    }
}

/// exports for C ABI
/// 
/// Most function return c_int, 0 means success, -1 means failure
pub mod exports;

/// keyboard related
pub mod keyboard;

/// mouse related
pub mod mouse;

/// window related
pub mod window;