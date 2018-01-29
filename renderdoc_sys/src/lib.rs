//! Raw FFI bindings to RenderDoc.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod app;

#[cfg(feature = "replay")]
pub mod replay;
