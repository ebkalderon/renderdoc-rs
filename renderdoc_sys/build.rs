extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let app = bindgen::Builder::default()
        .header("renderdoc/renderdoc/api/app/renderdoc_app.h")
        .whitelist_type("RENDERDOC_.*")
        .generate()
        .expect("Unable to generate app bindings!");

    app.write_to_file(out_path.join("app.rs"))
        .expect("Couldn't write app bindings!");

    let replay = bindgen::Builder::default()
        .header("replay/wrapper.h")
        .clang_args(&[
            "-x",
            "c++",
            "-std=c++11",
            "-DRENDERDOC_PLATFORM_LINUX",
            "-DRENDERDOC_WINDOWING_XLIB"
        ])
        .opaque_type("std::.*")
        .whitelist_function("GetNewUniqueId")
        .whitelist_function("RENDERDOC_.*")
        .whitelist_type(".*Description")
        .whitelist_type(".*Modification")
        .whitelist_type(".*Stage")
        .whitelist_type(".*State")
        .whitelist_type(".*Usage")
        .whitelist_type("CameraType")
        .whitelist_type("CaptureOptions")
        .whitelist_type("Counter.*")
        .whitelist_type("GlobalEnvironment")
        .whitelist_type("PathEntry")
        .whitelist_type("pRENDERDOC_.*")
        .whitelist_type("ReplayStatus")
        .whitelist_type("Shader.*")
        .whitelist_type("Texture.*")
        .whitelist_type("Window.*")
        .whitelist_type("XCBWindowData")
        .whitelist_type("XlibWindowData")
        .blacklist_type(".*IterContainer_.*")
        // Custom wrapper types.
        .whitelist_type("Camera")
        .whitelist_type("CaptureFile")
        .whitelist_type("RemoteServer")
        .whitelist_type("ReplayController")
        .whitelist_type("ReplayOutput")
        .whitelist_type("TargetControl")
        // .opaque_type("Camera")
        .generate_inline_functions(true)
        .generate()
        .expect("Unable to generate replay bindings!");

    replay
        .write_to_file(out_path.join("replay.rs"))
        .expect("Couldn't write replay bindings!");

    cc::Build::new()
        .include("replay")
        .include("renderdoc")
        .file("replay/src/Camera.cpp")
        .file("replay/src/CaptureFile.cpp")
        .file("replay/src/RemoteServer.cpp")
        .file("replay/src/ReplayController.cpp")
        .file("replay/src/ReplayOutput.cpp")
        .file("replay/src/TargetControl.cpp")
        .define("RENDERDOC_PLATFORM_LINUX", None)
        .define("RENDERDOC_WINDOWING_XLIB", None)
        .object("/usr/lib/librenderdoc.so")
        .pic(true)
        .cpp(true)
        .compile("librenderdoc.a");
}
