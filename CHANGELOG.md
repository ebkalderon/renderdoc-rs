# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
* Switch to Circle CI Rust 1.34.1 image.

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

## 0.1.0 - 2017-10-11
### Added
* Initial crate release.
* In-application API bindings, supporting versions 1.0 to 1.1.
* Type-safe version requests and downgrading.
* Convenient conversions for `winit::VirtualKeyCode` into RenderDoc `InputButton`.

[Unreleased]: https://github.com/ebkalderon/renderdoc-rs/compare/v0.6.0...HEAD
[0.6.0]: https://github.com/ebkalderon/renderdoc-rs/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/ebkalderon/renderdoc-rs/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/ebkalderon/renderdoc-rs/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/ebkalderon/renderdoc-rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/ebkalderon/renderdoc-rs/compare/v0.1.0...v0.2.0
