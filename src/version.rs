//! API versions.

#![allow(missing_docs)]

use {CaptureOption, DevicePointer, OverlayBits, InputButton, WindowHandle};
use entry::{EntryV100, EntryV110};

use std::ffi::{CStr, CString};
use std::mem;
use std::path::Path;

pub trait RenderDocV100: Sized {
    unsafe fn entry_v100(&self) -> &EntryV100;

    fn get_api_version(&self) -> (u32, u32, u32) {
        unsafe {
            let (mut major, mut minor, mut patch) = (0, 0, 0);
            (self.entry_v100().get_api_version)(&mut major, &mut minor, &mut patch);
            (major as u32, minor as u32, patch as u32)
        }
    }

    fn set_capture_option_f32(&mut self, opt: CaptureOption, val: f32) {
        let err = unsafe { (self.entry_v100().set_capture_option_f32)(opt, val) };
        assert_eq!(err, 1);
    }

    fn set_capture_option_u32(&mut self, opt: CaptureOption, val: u32) {
        let err = unsafe { (self.entry_v100().set_capture_option_u32)(opt, val) };
        assert_eq!(err, 1);
    }

    fn get_capture_option_f32(&self, opt: CaptureOption) -> f32 {
        use std::f32::MAX;
        let val = unsafe { (self.entry_v100().get_capture_option_f32)(opt) };
        assert_ne!(val, -MAX);
        val
    }

    fn get_capture_option_u32(&self, opt: CaptureOption) -> u32 {
        use std::u32::MAX;
        let val = unsafe { (self.entry_v100().get_capture_option_u32)(opt) };
        assert_ne!(val, MAX);
        val
    }

    fn set_capture_keys<I: Into<InputButton> + Clone>(&mut self, keys: &[I]) {
        unsafe {
            let k: Vec<_> = keys.iter().cloned().map(|k| k.into()).collect();
            (self.entry_v100().set_capture_keys)(k.as_ptr(), k.len() as i32)
        }
    }

    fn set_focus_toggle_keys<I: Into<InputButton> + Clone>(&mut self, keys: &[I]) {
        unsafe {
            let k: Vec<_> = keys.iter().cloned().map(|k| k.into()).collect();
            (self.entry_v100().set_focus_toggle_keys)(k.as_ptr(), k.len() as i32)
        }
    }

    unsafe fn shutdown(self) {
        (self.entry_v100().shutdown)();
    }

    fn unload_crash_handler(&mut self) {
        unsafe {
            (self.entry_v100().unload_crash_handler)();
        }
    }

    fn get_overlay_bits(&self) -> OverlayBits {
        unsafe { (self.entry_v100().get_overlay_bits)() }
    }

    fn mask_overlay_bits(&mut self, and: OverlayBits, or: OverlayBits) {
        unsafe {
            (self.entry_v100().mask_overlay_bits)(and, or);
        }
    }

    fn get_log_file_path_template(&self) -> &str {
        unsafe {
            let raw = (self.entry_v100().get_log_file_path_template)();
            CStr::from_ptr(raw).to_str().unwrap()
        }
    }

    fn set_log_file_path_template<P: AsRef<Path>>(&mut self, path_template: P) {
        unsafe {
            let bytes = mem::transmute(path_template.as_ref().as_os_str());
            let cstr = CStr::from_bytes_with_nul_unchecked(bytes);
            (self.entry_v100().set_log_file_path_template)(cstr.as_ptr());
        }
    }

    fn get_num_captures(&self) -> u32 {
        unsafe { (self.entry_v100().get_num_captures)() }
    }

    fn get_capture(&self, index: u32) -> Option<(String, u64)> {
        unsafe {
            let mut len = self.get_log_file_path_template().len() as u32 + 128;
            let mut path = Vec::with_capacity(len as usize);
            let mut time = 0u64;

            if (self.entry_v100().get_capture)(index, path.as_mut_ptr(), &mut len, &mut time) == 1 {
                let raw_path = CString::from_raw(path.as_mut_ptr());
                let mut path = raw_path.into_string().unwrap();
                path.shrink_to_fit();

                Some((path, time))
            } else {
                None
            }
        }
    }

    fn trigger_capture(&mut self) {
        unsafe {
            (self.entry_v100().trigger_capture)();
        }
    }

    fn is_target_control_connected(&self) -> bool {
        unsafe { (self.entry_v100().is_target_control_connected)() == 1 }
    }

    fn launch_replay_ui<C: Into<Option<&'static str>>>(&self, cmd_line: C) -> Result<u32, ()> {
        unsafe {
            let with_cmd = cmd_line.into();
            let (enabled, text) = if let Some(ref cmd) = with_cmd {
                let bytes = cmd.as_bytes();
                (1, CStr::from_bytes_with_nul_unchecked(bytes))
            } else {
                (0, Default::default())
            };

            match (self.entry_v100().launch_replay_ui)(enabled, text.as_ptr()) {
                0 => Err(()),
                pid => Ok(pid),
            }
        }
    }

    fn set_active_window(&mut self, dev: DevicePointer, win: WindowHandle) {
        unsafe {
            (self.entry_v100().set_active_window)(dev, win);
        }
    }

    fn start_frame_capture(&mut self, dev: DevicePointer, win: WindowHandle) {
        unsafe {
            (self.entry_v100().start_frame_capture)(dev, win);
        }
    }

    fn is_frame_capturing(&self) -> bool {
        unsafe { (self.entry_v100().is_frame_capturing)() == 1 }
    }

    fn end_frame_capture(&mut self, dev: DevicePointer, win: WindowHandle) {
        unsafe {
            (self.entry_v100().end_frame_capture)(dev, win);
        }
    }
}

pub trait RenderDocV110: RenderDocV100 {
    unsafe fn entry_v110(&self) -> &EntryV110;

    fn trigger_multi_frame_capture(&self, num_frames: u32) {
        unsafe {
            (self.entry_v110().trigger_multi_frame_capture)(num_frames);
        }
    }
}
