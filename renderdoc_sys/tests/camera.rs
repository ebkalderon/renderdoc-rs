extern crate renderdoc_sys;

#[cfg(feature = "replay")]
mod camera {
    use super::renderdoc_sys::replay::*;

    #[test]
    fn arcball() {
        unsafe {
            let mut cam = RENDERDOC_InitCamera(CameraType_Arcball);
            cam.SetArcballDistance(12.0);

            let pos = cam.GetPosition();
            assert_eq!(pos.x, 0.0);
            assert_eq!(pos.y, 0.0);
            assert_eq!(pos.z, 0.0);
            assert_eq!(pos.w, 1.0);

            cam.SetPosition(12.0, 6.4, -3.0);

            let pos = cam.GetPosition();
            assert_eq!(pos.x, 12.0);
            assert_eq!(pos.y, 6.4);
            assert_eq!(pos.z, -3.0);
            assert_eq!(pos.w, 1.0);

            let fwd = cam.GetForward();
            assert_eq!(fwd.x, 0.0);
            assert_eq!(fwd.y, 0.0);
            assert_eq!(fwd.z, 0.0);
            assert_eq!(fwd.w, 1.0);

            let right = cam.GetRight();
            assert_eq!(right.x, 0.0);
            assert_eq!(right.y, 0.0);
            assert_eq!(right.z, 0.0);
            assert_eq!(right.w, 1.0);

            let up = cam.GetUp();
            assert_eq!(up.x, 0.0);
            assert_eq!(up.y, 0.0);
            assert_eq!(up.z, 0.0);
            assert_eq!(up.w, 1.0);

            cam.Shutdown();
        }
    }

    #[test]
    fn fpslook() {
        unsafe {
            let mut cam = RENDERDOC_InitCamera(CameraType_FPSLook);
            cam.SetFPSRotation(2.0, -1.0, 9.0);

            let pos = cam.GetPosition();
            assert_eq!(pos.x, 0.0);
            assert_eq!(pos.y, 0.0);
            assert_eq!(pos.z, 0.0);
            assert_eq!(pos.w, 1.0);

            cam.SetPosition(12.0, 6.4, -3.0);

            let pos = cam.GetPosition();
            assert_eq!(pos.x, 12.0);
            assert_eq!(pos.y, 6.4);
            assert_eq!(pos.z, -3.0);
            assert_eq!(pos.w, 1.0);

            let fwd = cam.GetForward();
            assert_eq!(fwd.x, 0.0);
            assert_eq!(fwd.y, 0.0);
            assert_eq!(fwd.z, 0.0);
            assert_eq!(fwd.w, 1.0);

            let right = cam.GetRight();
            assert_eq!(right.x, 0.0);
            assert_eq!(right.y, 0.0);
            assert_eq!(right.z, 0.0);
            assert_eq!(right.w, 1.0);

            let up = cam.GetUp();
            assert_eq!(up.x, 0.0);
            assert_eq!(up.y, 0.0);
            assert_eq!(up.z, 0.0);
            assert_eq!(up.w, 1.0);

            cam.Shutdown();
        }
    }
}
