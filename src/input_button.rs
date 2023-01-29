use renderdoc_sys::RENDERDOC_InputButton;

/// A keyboard key.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
#[repr(u32)]
pub enum InputButton {
    Key0 = renderdoc_sys::eRENDERDOC_Key_0,
    Key1 = renderdoc_sys::eRENDERDOC_Key_1,
    Key2 = renderdoc_sys::eRENDERDOC_Key_2,
    Key3 = renderdoc_sys::eRENDERDOC_Key_3,
    Key4 = renderdoc_sys::eRENDERDOC_Key_4,
    Key5 = renderdoc_sys::eRENDERDOC_Key_5,
    Key6 = renderdoc_sys::eRENDERDOC_Key_6,
    Key7 = renderdoc_sys::eRENDERDOC_Key_7,
    Key8 = renderdoc_sys::eRENDERDOC_Key_8,
    Key9 = renderdoc_sys::eRENDERDOC_Key_9,

    A = renderdoc_sys::eRENDERDOC_Key_A,
    B = renderdoc_sys::eRENDERDOC_Key_B,
    C = renderdoc_sys::eRENDERDOC_Key_C,
    D = renderdoc_sys::eRENDERDOC_Key_D,
    E = renderdoc_sys::eRENDERDOC_Key_E,
    F = renderdoc_sys::eRENDERDOC_Key_F,
    G = renderdoc_sys::eRENDERDOC_Key_G,
    H = renderdoc_sys::eRENDERDOC_Key_H,
    I = renderdoc_sys::eRENDERDOC_Key_I,
    J = renderdoc_sys::eRENDERDOC_Key_J,
    K = renderdoc_sys::eRENDERDOC_Key_K,
    L = renderdoc_sys::eRENDERDOC_Key_L,
    M = renderdoc_sys::eRENDERDOC_Key_M,
    N = renderdoc_sys::eRENDERDOC_Key_N,
    O = renderdoc_sys::eRENDERDOC_Key_O,
    P = renderdoc_sys::eRENDERDOC_Key_P,
    Q = renderdoc_sys::eRENDERDOC_Key_Q,
    R = renderdoc_sys::eRENDERDOC_Key_R,
    S = renderdoc_sys::eRENDERDOC_Key_S,
    T = renderdoc_sys::eRENDERDOC_Key_T,
    U = renderdoc_sys::eRENDERDOC_Key_U,
    V = renderdoc_sys::eRENDERDOC_Key_V,
    W = renderdoc_sys::eRENDERDOC_Key_W,
    X = renderdoc_sys::eRENDERDOC_Key_X,
    Y = renderdoc_sys::eRENDERDOC_Key_Y,
    Z = renderdoc_sys::eRENDERDOC_Key_Z,

    Divide = renderdoc_sys::eRENDERDOC_Key_Divide,
    Multiply = renderdoc_sys::eRENDERDOC_Key_Multiply,
    Subtract = renderdoc_sys::eRENDERDOC_Key_Subtract,
    Plus = renderdoc_sys::eRENDERDOC_Key_Plus,

    F1 = renderdoc_sys::eRENDERDOC_Key_F1,
    F2 = renderdoc_sys::eRENDERDOC_Key_F2,
    F3 = renderdoc_sys::eRENDERDOC_Key_F3,
    F4 = renderdoc_sys::eRENDERDOC_Key_F4,
    F5 = renderdoc_sys::eRENDERDOC_Key_F5,
    F6 = renderdoc_sys::eRENDERDOC_Key_F6,
    F7 = renderdoc_sys::eRENDERDOC_Key_F7,
    F8 = renderdoc_sys::eRENDERDOC_Key_F8,
    F9 = renderdoc_sys::eRENDERDOC_Key_F9,
    F10 = renderdoc_sys::eRENDERDOC_Key_F10,
    F11 = renderdoc_sys::eRENDERDOC_Key_F11,
    F12 = renderdoc_sys::eRENDERDOC_Key_F12,

    Home = renderdoc_sys::eRENDERDOC_Key_Home,
    End = renderdoc_sys::eRENDERDOC_Key_End,
    Insert = renderdoc_sys::eRENDERDOC_Key_Insert,
    Delete = renderdoc_sys::eRENDERDOC_Key_Delete,
    PageUp = renderdoc_sys::eRENDERDOC_Key_PageUp,
    PageDn = renderdoc_sys::eRENDERDOC_Key_PageDn,

    Backspace = renderdoc_sys::eRENDERDOC_Key_Backspace,
    Tab = renderdoc_sys::eRENDERDOC_Key_Tab,
    PrtScrn = renderdoc_sys::eRENDERDOC_Key_PrtScrn,
    Pause = renderdoc_sys::eRENDERDOC_Key_Pause,
}

/// A trait implemented by types containing zero or more `InputButton`s.
pub trait AsInputButtons {
    /// Returns a pointer to the beginning of the array.
    fn as_ptr(&self) -> *const RENDERDOC_InputButton;

    /// Returns the number of elements in the slice.
    fn len(&self) -> i32;
}

impl<I: AsInputButtons> AsInputButtons for &'_ I {
    #[inline]
    fn as_ptr(&self) -> *const RENDERDOC_InputButton {
        (*self).as_ptr()
    }

    #[inline]
    fn len(&self) -> i32 {
        (*self).len()
    }
}

impl AsInputButtons for InputButton {
    #[inline]
    fn as_ptr(&self) -> *const RENDERDOC_InputButton {
        self as *const InputButton as *const _
    }

    #[inline]
    fn len(&self) -> i32 {
        1
    }
}

impl AsInputButtons for Option<InputButton> {
    #[inline]
    fn as_ptr(&self) -> *const RENDERDOC_InputButton {
        self.as_ref()
            .map(|v| v as *const InputButton as *const _)
            .unwrap_or_else(std::ptr::null)
    }

    #[inline]
    fn len(&self) -> i32 {
        self.map(|_| 1).unwrap_or(0)
    }
}

impl AsInputButtons for [InputButton] {
    #[inline]
    fn as_ptr(&self) -> *const RENDERDOC_InputButton {
        (*self).as_ptr() as *const _
    }

    #[inline]
    fn len(&self) -> i32 {
        (*self).len() as i32
    }
}

impl<const N: usize> AsInputButtons for [InputButton; N] {
    #[inline]
    fn as_ptr(&self) -> *const RENDERDOC_InputButton {
        self.as_ref().as_ptr() as *const _
    }

    #[inline]
    fn len(&self) -> i32 {
        self.as_ref().len() as i32
    }
}

impl AsInputButtons for Vec<InputButton> {
    #[inline]
    fn as_ptr(&self) -> *const RENDERDOC_InputButton {
        self.as_ptr() as *const _
    }

    #[inline]
    fn len(&self) -> i32 {
        self.len() as i32
    }
}
