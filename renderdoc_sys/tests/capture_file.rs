extern crate renderdoc_sys;

#[cfg(feature = "replay")]
mod capture_file {
    use super::renderdoc_sys::replay::*;

    #[test]
    fn read_filename() {
        unsafe {
            use std::ffi::CString;

            let cap = concat!(env!("CARGO_MANIFEST_DIR"), "/triangle.rdc");
            let raw = CString::new(cap.as_bytes()).expect("Found null byte");
            let mut file = RENDERDOC_OpenCaptureFile(raw.as_ptr());

            assert_eq!(cap, raw.to_str().expect("Unable to convert to &str"));

            file.Shutdown();
        }
    }
}
