use error::*;
use std::ptr;

pub mod ffi {
    use std::libc::{c_int, uint32_t, c_char};
    pub struct SDL_Window;

    pub enum SDL_WindowFlags {
        SDL_WINDOW_FULLSCREEN = 0x00000001,
        SDL_WINDOW_OPENGL = 0x00000002,
        SDL_WINDOW_SHOWN = 0x00000004,
        SDL_WINDOW_HIDDEN = 0x00000008,
        SDL_WINDOW_BORDERLESS = 0x00000010,
        SDL_WINDOW_RESIZABLE = 0x00000020,
        SDL_WINDOW_MINIMIZED = 0x00000040,
        SDL_WINDOW_MAXIMIZED = 0x00000080,
        SDL_WINDOW_INPUT_GRABBED = 0x00000100,
        SDL_WINDOW_INPUT_FOCUS = 0x00000200,
        SDL_WINDOW_MOUSE_FOCUS = 0x00000400,
        SDL_WINDOW_FULLSCREEN_DESKTOP = 0x00001001,
        SDL_WINDOW_FOREIGN = 0x00000800
    }

    pub enum SDL_WindowEventID {
        SDL_WINDOWEVENT_NONE = 0_u8,
        SDL_WINDOWEVENT_SHOWN = 1_u8,
        SDL_WINDOWEVENT_HIDDEN = 2_u8,
        SDL_WINDOWEVENT_EXPOSED = 3_u8,
        SDL_WINDOWEVENT_MOVED = 4_u8,
        SDL_WINDOWEVENT_RESIZED = 5_u8,
        SDL_WINDOWEVENT_SIZE_CHANGED = 6_u8,
        SDL_WINDOWEVENT_MINIMIZED = 7_u8,
        SDL_WINDOWEVENT_MAXIMIZED = 8_u8,
        SDL_WINDOWEVENT_RESTORED = 9_u8,
        SDL_WINDOWEVENT_ENTER = 10_u8,
        SDL_WINDOWEVENT_LEAVE = 11_u8,
        SDL_WINDOWEVENT_FOCUS_GAINED = 12_u8,
        SDL_WINDOWEVENT_FOCUS_LOST = 13_u8,
        SDL_WINDOWEVENT_CLOSE = 14_u8,
    }

    externfn!(fn SDL_CreateWindow(title: *c_char, 
                                  x: c_int, y: c_int, 
                                  w: c_int, h: c_int, 
                                  flags: uint32_t) -> *SDL_Window)

    externfn!(fn SDL_DestroyWindow(window: *SDL_Window))
    externfn!(fn SDL_GetWindowFromID(id: uint32_t) -> *SDL_Window)
}

pub enum WindowFlags {
    FullScreen = ffi::SDL_WINDOW_FULLSCREEN as u32,
    OpenGL = ffi::SDL_WINDOW_OPENGL as u32,
    Shown = ffi::SDL_WINDOW_SHOWN as u32,
    Hidden = ffi::SDL_WINDOW_HIDDEN as u32,
    Borderless = ffi::SDL_WINDOW_BORDERLESS as u32,
    Resizable = ffi::SDL_WINDOW_RESIZABLE as u32,
    Minimized = ffi::SDL_WINDOW_MINIMIZED as u32,
    Maximized = ffi::SDL_WINDOW_MAXIMIZED as u32,
    InputGrabbed = ffi::SDL_WINDOW_INPUT_GRABBED as u32,
    InputFocus = ffi::SDL_WINDOW_INPUT_FOCUS as u32,
    MouseFocus = ffi::SDL_WINDOW_MOUSE_FOCUS as u32,
    FullScreenDesktop = ffi::SDL_WINDOW_FULLSCREEN_DESKTOP as u32,
    Foreign = ffi::SDL_WINDOW_FOREIGN as u32,
}

#[deriving(FromPrimitive)]
pub enum WindowEventID {
    NoneEvent = ffi::SDL_WINDOWEVENT_NONE as u8,
    ShownEvent = ffi::SDL_WINDOWEVENT_SHOWN as u8,
    HiddenEvent = ffi::SDL_WINDOWEVENT_HIDDEN as u8,
    ExposedEvent = ffi::SDL_WINDOWEVENT_EXPOSED as u8,
    MovedEvent = ffi::SDL_WINDOWEVENT_MOVED as u8,
    ResizedEvent = ffi::SDL_WINDOWEVENT_RESIZED as u8,
    SizeChangedEvent = ffi::SDL_WINDOWEVENT_SIZE_CHANGED as u8,
    MinimizedEvent = ffi::SDL_WINDOWEVENT_MINIMIZED as u8,
    MaximizedEvent = ffi::SDL_WINDOWEVENT_MAXIMIZED as u8,
    RestoredEvent = ffi::SDL_WINDOWEVENT_RESTORED as u8,
    EnterEvent = ffi::SDL_WINDOWEVENT_ENTER as u8,
    LeaveEvent = ffi::SDL_WINDOWEVENT_LEAVE as u8,
    FocusGainedEvent = ffi::SDL_WINDOWEVENT_FOCUS_GAINED as u8,
    FocusLostEvent = ffi::SDL_WINDOWEVENT_FOCUS_LOST as u8,
    CloseEvent = ffi::SDL_WINDOWEVENT_CLOSE as u8,
}

pub struct Window {
    raw: *ffi::SDL_Window,
    owned: bool,
}

impl Drop for Window {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ffi::SDL_DestroyWindow(self.raw);
            }
        }
    }
}

impl Window {
    pub fn new(title: &str, 
               x: i32, y: i32,
               w: i32, h: i32,
               flags: &[WindowFlags]) -> Result<~Window, ~str> {
        let f = do flags.iter().fold(0) |all, it| {
            all | *it as u32
        };

        unsafe {
            let raw = do title.with_c_str |buf| {
                ffi::SDL_CreateWindow(buf, x, y, w, h, f)
            };

            if ptr::is_null(raw) {
                Err(get_error())
            } else {
                Ok(~Window{ raw: raw, owned: true })
            }
        }
    }

    pub fn get_from_id(id: u32) -> Result<~Window, ~str> {
        unsafe {
            let raw = ffi::SDL_GetWindowFromID(id);

            if ptr::is_null(raw) {
                Err(get_error())
            } else {
                Ok(~Window{raw: raw, owned: false})
            }
        }
    }
}

