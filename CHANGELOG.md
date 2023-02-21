# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.11.0] - 2023-02-20

### Changed

* Update `renderdoc-sys` dependency to 1.0.0.
* Update `float-cmp` dependency to 0.9.
* Update optional `glutin` dependency to 0.30.
* Update optional `winit` dependency to 0.28.
* Improve crate-level documentation.

### Fixed

* Fix loading strategy of `renderdoc` library (PR #140).
* Fix typo in `start_frame_capture` and `end_frame_capture` docs (PR #122).
* Fix undefined behavior and crash in `get_capture` (PR #143).

## [0.10.1] - 2021-02-10

### Changed

* Bump `libloading` dependency to 0.7.

## [0.10.0] - 2020-12-17

### Changed

* Bump `glutin` dependency to 0.26.

## [0.9.1] - 2020-08-05

### Changed

* Allow global synchronized access to RenderDoc (see #79).
* Exclude `vendor` subdirectory from `renderdoc-sys` crate.
* Enable all crate features when generating docs for docs.rs.

### Fixed

* Expose `V141` version selector.

## [0.9.0] - 2020-05-17

### Added

* Implement preliminary support for API version 1.4.1 (see #93).

### Changed

* Bump `glutin` dependency to 0.24, disable integration by default (PR #94).

## [0.8.1] - 2020-05-01

### Fixed

* Fix copy-paste documentation mistake for `end_frame_capture()`.
* Fix formatting for `unload_crash_handler()` docs.
* Fix subtle spacing issue around `$PATH` in docs.

## [0.8.0] - 2020-05-01

### Added

* Add dedicated `Error` type to be used throughout the library.

### Changed

* Expand API documentation and improve existing documentation quality (PR #81).
* Accept and return `PathBuf` or `&Path` in places instead of String and `&str`
  (PR #82).
* Accept `Into<String>` and `Into<PathBuf>` instead of `AsRef` in places where
  we are going to be allocating anyway (PR #82).
* Return `std::time::SystemTime` instead of a raw `u64` in `get_capture()`.
* Convert crate to Rust 2018 edition.
* Bump `float-cmp` dependency to 0.7.
* Bump `libloading` dependency to 0.6.
* Switch to Circle CI Rust 1.40.0 image.

### Deprecated

* Mark `get_log_file_path_template()` and `set_log_file_path_template()` as
  deprecated for all RenderDoc API versions after 1.1.2 (PR #83).

## [0.7.1] - 2019-10-07

### Fixed

* Fix build without `glutin` enabled (PR #69).

## [0.7.0] - 2019-08-23

### Added

* Write more doc comments and add doc tests.

### Changed

* Bump `float-cmp` dependency to 0.5.
* Switch to Circle CI Rust 1.33.0 image.
* Change error type of `launch_replay_ui()` from `()` to `String`.
* Mark `Deref` block as `#[doc(hidden)]` for cleaner generated docs.

### Removed

* Remove internal `renderdoc-derive` crate in favor of declarative macro.
* Eliminate unnecessary `unsafe` blocks.

### Fixed

* Define `CaptureOption`, `InputButton`, and `OverlayBits` in terms of
  `renderdoc-sys` types.
* Add missing discriminant values to `InputButton` enum.
* Fix broken Windows build (PR #61).

## [0.6.0] - 2019-05-19

### Added

* Redesign crate to use inherent impls over traits (PR #35).
* Add `HasPrevious` trait to recursively determine version compatibility at
  compile-time.

### Changed

* Rename `Version` enum to `VersionCode` and `ApiVersion` trait to `Version`.
* Use a single `Entry` type, since the aliases point to the same struct.
* Update crate metadata and improve documentation.
* Manually implement `Debug`, derive `Eq`, `Hash`, `PartialEq` for most types
  (PR #41).
* Apply Clippy suggestions (PR #43).

### Deprecated

* Mark `is_remote_access_connected()` as deprecated for all RenderDoc API
  versions after 1.1.1 (PR #42).

### Removed

* Remove `prelude` module.
* Remove `RenderDocV###` traits.
* Remove `RenderDocV###` trait boilerplate code from `renderdoc-derive`.
* Remove unused `__uint32_t` and `__uint64_t` type aliases from `renderdoc-sys`
  (PR #39).

## [0.5.0] - 2019-05-19

### Added

* Add CI and documentation badges.
* Implement support for API versions 1.3.0 and 1.4.0.
* Allow string slices with lifetimes in `set_capture_file_comments()`.

### Changed

* Bump `glutin` dependency to 0.21.
* Bump `gfx` dev-dependency to 0.18.1.
* Bump `gfx_window_glutin` dev-dependency to 0.31.
* Upgrade CircleCI Rust image to 1.34.1.
* Convert top-level crate to workspace.
* Clean up `renderdoc-sys` crate layout.
* Minor code formatting tweaks.

### Fixed

* Switch `set_capture_file_comments()` and `trigger_multi_frame_capture()` to
  take `&mut self` (PR #32).
* Unimplement `Clone`, `Send`, and `Sync` for `RenderDoc` struct (PR #29).
* Correct default setting in the `get_set_capture_option()` unit test.
* Fix improperly designed `launch_replay_ui()` method, update `triangle` example
  to match.
* Set correct RenderDoc library path for Android clients.
* Add missing trait re-exports to `prelude` module (PR #31).
* Fix erroneous doc comments (PR #24).

## [0.4.0] - 2018-09-16

### Added

* Create `renderdoc-sys` crate for raw FFI bindings.
* Create `renderdoc-derive` crate for internal codegen.
* Add support for RenderDoc API 1.1.1, 1.1.2, and 1.2.0.

### Changed

* Switch to `libloading` from `shared_library`.
* Update `triangle` example to the latest `glutin` API.
* Bump dependencies.

## [0.3.0] - 2018-06-01

### Changed

* Update existing dependencies (PR #3).

## [0.2.0] - 2017-12-15

### Added

* Convenient conversions for `glutin::Context`, `winapi::D3D11Device`,
  `winapi::D3D12Device`, and `winapi::windef::HGLRC` into RenderDoc
  `DevicePointer`.

### Changed

* Update existing dependencies.
* Optionally depend on `glutin` in place of `winit`.
* Depend on `wio` for Windows targets.

### Fixed

* Missing byte in `SHADER_MAGIC_DEBUG_VALUE_STRUCT` broke Windows builds.

## [0.1.0] - 2017-10-11

### Added

* Initial crate release.
* In-application API bindings, supporting versions 1.0 to 1.1.
* Type-safe version requests and downgrading.
* Convenient conversions for `winit::VirtualKeyCode` into RenderDoc `InputButton`.

[Unreleased]: https://github.com/ebkalderon/renderdoc-rs/compare/v0.11.0...HEAD
[0.11.0]: https://github.com/ebkalderon/renderdoc-rs/compare/v0.10.1...v0.11.0
[0.10.1]: https://github.com/ebkalderon/renderdoc-rs/compare/v0.10.0...v0.10.1
[0.10.0]: https://github.com/ebkalderon/renderdoc-rs/compare/v0.9.1...v0.10.0
[0.9.1]: https://github.com/ebkalderon/renderdoc-rs/compare/v0.9.0...v0.9.1
[0.9.0]: https://github.com/ebkalderon/renderdoc-rs/compare/v0.8.1...v0.9.0
[0.8.1]: https://github.com/ebkalderon/renderdoc-rs/compare/v0.8.0...v0.8.1
[0.8.0]: https://github.com/ebkalderon/renderdoc-rs/compare/v0.7.1...v0.8.0
[0.7.1]: https://github.com/ebkalderon/renderdoc-rs/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/ebkalderon/renderdoc-rs/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/ebkalderon/renderdoc-rs/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/ebkalderon/renderdoc-rs/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/ebkalderon/renderdoc-rs/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/ebkalderon/renderdoc-rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/ebkalderon/renderdoc-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/ebkalderon/renderdoc-rs/releases/tag/v0.1.0
