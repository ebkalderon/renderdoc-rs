# renderdoc-rs

[![Build Status][build-badge]][build-url]
[![Crates.io][crate-badge]][crate-url]
[![Documentation][docs-badge]][docs-url]

[build-badge]: https://circleci.com/gh/ebkalderon/renderdoc-rs.svg?style=shield
[build-url]: https://circleci.com/gh/ebkalderon/renderdoc-rs
[crate-badge]: https://img.shields.io/crates/v/renderdoc.svg
[crate-url]: https://crates.io/crates/renderdoc
[docs-badge]: https://docs.rs/renderdoc/badge.svg
[docs-url]: https://docs.rs/renderdoc

Rust bindings to [RenderDoc], a popular graphics debugger.

[RenderDoc]: https://renderdoc.org/

RenderDoc is a free and open source graphics debugging tool. RenderDoc allows
game developers to take frame captures of their applications, replay them,
examine the graphics pipeline state, and potentially identify nasty graphics
bugs.

These bindings require that RenderDoc be installed on the target machine, with
either `renderdoc.dll` or `librenderdoc.so` visible from your `$PATH`.

For more details on how to use this API to integrate your game or renderer with
the RenderDoc profiler, consult the [in-application API][in-app] documentation.

[in-app]: https://renderdoc.org/docs/in_application_api.html

## Example

```rust
use renderdoc::{RenderDoc, V100, V110};

fn main() {
    let mut rd: RenderDoc<V110> = RenderDoc::new().expect("Unable to connect");

    let (major, minor, patch) = rd.get_api_version();
    assert_eq!(major, 1u32);
    assert!(minor >= 1u32);

    // When a certain key is pressed, trigger a single-frame capture like this.
    rd.trigger_capture();

    // If you specify version `V110` or newer, you can trigger a multi-frame
    // capture like this.
    rd.trigger_multi_frame_capture(3);

    // Query the details of an existing capture like this.
    match rd.get_capture(0) {
        Some((path, capture_time)) => println!("ID: 0, Path: {}, Captured: {:?}", path, capture_time),
        None => println!("No capture found with ID of 0!"),
    }

    // Downgrade your effective API version at run-time like this.
    let mut rd: RenderDoc<V100> = rd.into();

    // Now this line will no longer compile!
    // rd.trigger_multi_frame_capture(3);
}
```

Working examples are available in the `examples` directory.

## License

`renderdoc-rs` is free and open source software distributed under the terms of
either the [MIT](LICENSE-MIT) or the [Apache 2.0](LICENSE-APACHE) license, at
your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
