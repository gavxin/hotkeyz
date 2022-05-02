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

mod exports;
mod keyboard;
mod mouse;
mod window;