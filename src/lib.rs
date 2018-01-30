//! RenderDoc application bindings for Rust

#![deny(missing_docs)]

#[cfg(any(target_os = "macos", target_os = "ios"))]
compile_error!("RenderDoc does not support this platform.");

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
extern crate renderdoc_sys;
extern crate shared_library;

#[cfg(feature = "glutin")]
extern crate glutin;
#[cfg(target_os = "windows")]
extern crate winapi;
#[cfg(target_os = "windows")]
extern crate wio;

pub mod app;
