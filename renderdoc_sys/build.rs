extern crate bindgen;
extern crate cc;

use std::env;
use std::path::{Path, PathBuf};

#[cfg(unix)]
const SEARCH_PATH: &str = "/usr/lib";
#[cfg(windows)]
const SEARCH_PATH: &str = "C:\\Program Files (x64)\\RenderDoc";

fn main() {
    println!("cargo:rustc-link-search=native={}", SEARCH_PATH);
    println!("cargo:rustc-link-lib=renderdoc");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    if cfg!(not(apple)) {
        if cfg!(feature = "app") {
            gen_app_bindings(&out_path);
        }

        if cfg!(feature = "replay") {
            gen_replay_bindings(&out_path);
        }
    }
}

fn gen_app_bindings<P: AsRef<Path>>(out_path: P) {
    let app = bindgen::Builder::default()
        .header("src/renderdoc/renderdoc/api/app/renderdoc_app.h")
        .whitelist_type("RENDERDOC_.*")
        .blacklist_type("__.*")
        .generate()
        .expect("Unable to generate app bindings!");

    app.write_to_file(out_path.as_ref().join("app.rs"))
        .expect("Couldn't write app bindings!");
}

fn gen_replay_bindings<P: AsRef<Path>>(out_path: P) {
    #[cfg(unix)]
    let platform_args = [
        "-DRENDERDOC_PLATFORM_LINUX",
        "-DRENDERDOC_WINDOWING_XLIB"
    ];

    #[cfg(windows)]
    let platform_args = ["-DRENDERDOC_PLATFORM_WIN32"];

    let replay = bindgen::Builder::default()
        .header("src/replay/wrapper.h")
        .clang_args(&[
            "-x",
            "c++",
            "-std=c++11",
        ])
        .clang_args(&platform_args)
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
        .whitelist_type("Shader.*")
        .whitelist_type("Texture.*")
        .whitelist_type("Window.*")
        .whitelist_type("XCBWindowData")
        .whitelist_type("XlibWindowData")
        .blacklist_type(".*IterContainer_.*")
        // Custom wrapper types.
        .whitelist_function("RENDERDOC::.*")
        .whitelist_type("Camera")
        .whitelist_type("CaptureFile")
        .whitelist_type("RemoteServer")
        .whitelist_type("ReplayController")
        .whitelist_type("ReplayOutput")
        .whitelist_type("TargetControl")
        .generate_inline_functions(true)
        .generate()
        .expect("Unable to generate replay bindings!");

    replay
        .write_to_file(out_path.as_ref().join("replay.rs"))
        .expect("Couldn't write replay bindings!");

    let mut build = cc::Build::new();

    #[cfg(unix)]
    build
        .define("RENDERDOC_PLATFORM_LINUX", None)
        .define("RENDERDOC_WINDOWING_XLIB", None);

    #[cfg(windows)]
    build.define("RENDERDOC_PLATFORM_WINDOWS", None);

    build
        .include("src/replay")
        .include("src/renderdoc")
        .file("src/replay/src/api.cpp")
        .file("src/replay/src/camera.cpp")
        .file("src/replay/src/capture_file.cpp")
        .file("src/replay/src/remote_server.cpp")
        .file("src/replay/src/replay_controller.cpp")
        .file("src/replay/src/replay_output.cpp")
        .file("src/replay/src/target_control.cpp")
        .object(library_path())
        .flag_if_supported("-L/usr/lib")
        .flag_if_supported("-lrenderdoc")
        .pic(true)
        .cpp(true)
        .compile("librenderdoc_wrap.a");
}

#[cfg(windows)]
fn library_path() -> PathBuf {
    Path::new(SEARCH_PATH).join("renderdoc.dll")
}

#[cfg(unix)]
fn library_path() -> PathBuf {
    Path::new(SEARCH_PATH).join("librenderdoc.so")
}
