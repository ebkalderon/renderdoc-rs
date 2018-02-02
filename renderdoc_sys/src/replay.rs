//! Raw FFI bindings to the replay API.

include!(concat!(env!("OUT_DIR"), "/replay.rs"));

#[cfg(test)]
mod replay {
    use super::*;

    #[test]
    fn camera_arcball() {
        unsafe {
            let mut cam = RENDERDOC_InitCamera(CameraType_Arcball);
            cam.SetArcballDistance(12.0f32);

            let pos = cam.GetPosition();
            assert_approx_eq!(pos.x, 0.0f32);
            assert_approx_eq!(pos.y, 0.0f32);
            assert_approx_eq!(pos.z, 0.0f32);
            assert_approx_eq!(pos.w, 1.0f32);

            cam.SetPosition(12.0f32, 6.4f32, -3.0f32);

            let pos = cam.GetPosition();
            assert_approx_eq!(pos.x, 12.0f32);
            assert_approx_eq!(pos.y, 6.4f32);
            assert_approx_eq!(pos.z, -3.0f32);
            assert_approx_eq!(pos.w, 1.0f32);

            let fwd = cam.GetForward();
            assert_approx_eq!(fwd.x, 0.0f32);
            assert_approx_eq!(fwd.y, 0.0f32);
            assert_approx_eq!(fwd.z, 0.0f32);
            assert_approx_eq!(fwd.w, 1.0f32);

            let right = cam.GetRight();
            assert_approx_eq!(right.x, 0.0f32);
            assert_approx_eq!(right.y, 0.0f32);
            assert_approx_eq!(right.z, 0.0f32);
            assert_approx_eq!(right.w, 1.0f32);

            let up = cam.GetUp();
            assert_approx_eq!(up.x, 0.0f32);
            assert_approx_eq!(up.y, 0.0f32);
            assert_approx_eq!(up.z, 0.0f32);
            assert_approx_eq!(up.w, 1.0f32);

            cam.Shutdown();
        }
    }

    #[test]
    fn camera_fpslook() {
        unsafe {
            let mut cam = RENDERDOC_InitCamera(CameraType_FPSLook);
            cam.SetFPSRotation(2.0f32, -1.0f32, 9.0f32);

            let pos = cam.GetPosition();
            assert_approx_eq!(pos.x, 0.0f32);
            assert_approx_eq!(pos.y, 0.0f32);
            assert_approx_eq!(pos.z, 0.0f32);
            assert_approx_eq!(pos.w, 1.0f32);

            cam.SetPosition(12.0f32, 6.4f32, -3.0f32);

            let pos = cam.GetPosition();
            assert_approx_eq!(pos.x, 12.0f32);
            assert_approx_eq!(pos.y, 6.4f32);
            assert_approx_eq!(pos.z, -3.0f32);
            assert_approx_eq!(pos.w, 1.0f32);

            let fwd = cam.GetForward();
            assert_approx_eq!(fwd.x, 0.0f32);
            assert_approx_eq!(fwd.y, 0.0f32);
            assert_approx_eq!(fwd.z, 0.0f32);
            assert_approx_eq!(fwd.w, 1.0f32);

            let right = cam.GetRight();
            assert_approx_eq!(right.x, 0.0f32);
            assert_approx_eq!(right.y, 0.0f32);
            assert_approx_eq!(right.z, 0.0f32);
            assert_approx_eq!(right.w, 1.0f32);

            let up = cam.GetUp();
            assert_approx_eq!(up.x, 0.0f32);
            assert_approx_eq!(up.y, 0.0f32);
            assert_approx_eq!(up.z, 0.0f32);
            assert_approx_eq!(up.w, 1.0f32);

            cam.Shutdown();
        }
    }

    #[test]
    fn capture_file() {
        unsafe {
            use std::ffi::{CStr, CString};

            let raw = CString::new("".as_bytes()).expect("Found trailing null");
            let mut file = RENDERDOC_OpenCaptureFile(raw.as_ptr());

            assert_eq!(ReplayStatus_FileIOFailed, file.OpenStatus());

            let filename = CStr::from_ptr(file.Filename());
            assert_eq!("", filename.to_str().expect("Failed to convert"));

            let driver = CStr::from_ptr(file.DriverName());
            assert_eq!("", driver.to_str().expect("Failed to convert"));

            file.Shutdown();
        }
    }
}
