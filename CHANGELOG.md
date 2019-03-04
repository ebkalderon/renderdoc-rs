# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
* Add CI and documentation badges.

### Changed
* Minor code formatting tweaks.
* Switch to Circle CI Rust 1.33.0 image.

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

[Unreleased]: https://github.com/ebkalderon/renderdoc-rs/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/ebkalderon/renderdoc-rs/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/ebkalderon/renderdoc-rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/ebkalderon/renderdoc-rs/compare/v0.1.0...v0.2.0
