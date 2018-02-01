//! Rust bindings to [RenderDoc], a popular graphics debugger.
//!
//! [RenderDoc]: https://renderdoc.org/
//!
//! RenderDoc is a free and open source graphics debugging tool. RenderDoc
//! allows game developers to take frame captures of their applications, replay
//! them, examine the graphics pipeline state, and potentially identify nasty
//! graphics bugs.
//!
//! These bindings require that RenderDoc be installed on the target machine,
//! with either `renderdoc.dll` or `librenderdoc.so` visible from your`PATH`.

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
#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate wio;

#[cfg(feature = "app")]
pub mod app;
#[cfg(feature = "replay")]
pub mod replay;
