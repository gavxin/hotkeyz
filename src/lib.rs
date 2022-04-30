#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate error_chain;
mod errors {
  error_chain! {
    foreign_links {
      Io(::std::io::Error);
    }
  }
}

mod keyboard;
