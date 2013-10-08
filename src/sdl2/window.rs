use std::libc::{c_int, c_float, uint32_t};
use std::ptr;
use std::str;
use std::cast;
use std::vec;
use error::*;

mod ffi;
mod rect;

#[deriving(Eq)]
pub enum GLAttr {
    GLRedSize = ffi::SDL_GL_RED_SIZE as int,
    GLGreenSize = ffi::SDL_GL_GREEN_SIZE as int,
    GLBlueSize = ffi::SDL_GL_BLUE_SIZE as int,
    GLAlphaSize = ffi::SDL_GL_ALPHA_SIZE as int,
    GLBufferSize = ffi::SDL_GL_BUFFER_SIZE as int,
    GLDoubleBuffer = ffi::SDL_GL_DOUBLEBUFFER as int,
    GLDepthSize = ffi::SDL_GL_DEPTH_SIZE as int,
    GLStencilSize = ffi::SDL_GL_STENCIL_SIZE as int,
    GLAccumRedSize = ffi::SDL_GL_ACCUM_RED_SIZE as int,
    GLAccumGreenSize = ffi::SDL_GL_ACCUM_GREEN_SIZE as int,
    GLAccumBlueSize = ffi::SDL_GL_ACCUM_BLUE_SIZE as int,
    GLAccumAlphaSize = ffi::SDL_GL_ACCUM_ALPHA_SIZE as int,
    GLStereo = ffi::SDL_GL_STEREO as int,
    GLMultiSampleBuffers = ffi::SDL_GL_MULTISAMPLEBUFFERS as int,
    GLMultiSampleSamples = ffi::SDL_GL_MULTISAMPLESAMPLES as int,
    GLAcceleratedVisual = ffi::SDL_GL_ACCELERATED_VISUAL as int,
    GLRetailedBacking = ffi::SDL_GL_RETAINED_BACKING as int,
    GLContextMajorVersion = ffi::SDL_GL_CONTEXT_MAJOR_VERSION as int,
    GLContextMinorVersion = ffi::SDL_GL_CONTEXT_MINOR_VERSION as int,
    GLContextEGL = ffi::SDL_GL_CONTEXT_EGL as int,
    GLContextFlags = ffi::SDL_GL_CONTEXT_FLAGS as int,
    GLContextProfileMask = ffi::SDL_GL_CONTEXT_PROFILE_MASK as int,
    GLShareWithCurrentContext = ffi::SDL_GL_SHARE_WITH_CURRENT_CONTEXT as int
}

#[deriving(Eq)]
pub enum WindowFlags {
    Fullscreen = ffi::SDL_WINDOW_FULLSCREEN as int,
    OpenGL = ffi::SDL_WINDOW_OPENGL as int,
    Shown = ffi::SDL_WINDOW_SHOWN as int,
    Hidden = ffi::SDL_WINDOW_HIDDEN as int,
    Borderless = ffi::SDL_WINDOW_BORDERLESS as int,
    Resizable = ffi::SDL_WINDOW_RESIZABLE as int,
    Minimized = ffi::SDL_WINDOW_MINIMIZED as int,
    Maximized = ffi::SDL_WINDOW_MAXIMIZED as int,
    InputGrabbed = ffi::SDL_WINDOW_INPUT_GRABBED as int,
    InputFocus = ffi::SDL_WINDOW_INPUT_FOCUS as int,
    MouseFocus = ffi::SDL_WINDOW_MOUSE_FOCUS as int,
    FullscreenDesktop = ffi::SDL_WINDOW_FULLSCREEN_DESKTOP as int,
    Foreign = ffi::SDL_WINDOW_FOREIGN as int
}

#[deriving(Eq)]
pub enum FullscreenType {
    FTOff = 0,
    FTTrue = Fullscreen as int,
    FTDesktop = FullscreenDesktop as int
}

#[deriving(Eq)]
pub enum WindowPos {
    PosUndefined,
    PosCentered,
    Positioned(int)
}

#[deriving(Eq)]
pub struct GLContext {
    raw: ffi::SDL_GLContext,
    owned: bool
}

impl Drop for GLContext {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ffi::SDL_GL_DeleteContext(self.raw)
            }
        }
    }
}


#[deriving(Eq)]
pub struct Window {
    raw: *ffi::SDL_Window,
    owned: bool
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
    pub fn new(title: &str, x: int, y: int, width: int, height: int, window_flags: &[WindowFlags]) -> Result<~Window, ~str> {
        let flags = window_flags.iter().fold(0u32, |flags, flag| { flags | *flag as u32 });

        unsafe {
            let raw = do title.with_c_str |buff| {
                ffi::SDL_CreateWindow(
                    buff,
                    x as c_int,
                    y as c_int,
                    width as c_int,
                    height as c_int,
                    flags
                )
            };

            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(~Window{ raw: raw, owned: true })
            }
        }
    }

    pub fn get_id(&self) -> u32 {
        unsafe { ffi::SDL_GetWindowID(self.raw) }
    }

    pub fn get_flags(&self) -> ~[WindowFlags] {
        let raw = unsafe { ffi::SDL_GetWindowFlags(self.raw) };
        wrap_window_flags(raw) 
    }

    pub fn set_title(&self, title: &str) {
        do title.with_c_str |buff| {
            unsafe { ffi::SDL_SetWindowTitle(self.raw, buff) }
        }
    }
    
    pub fn get_title(&self) -> ~str {
        unsafe {
            let cstr = ffi::SDL_GetWindowTitle(self.raw);
            str::raw::from_c_str(cast::transmute_copy(&cstr))
        }
    }
    // TODO: implement
    /*
    pub fn set_icon(&self, icon: &Surface) {
        unsafe { ffi::SDL_SetWindowIcon(self.raw, icon.raw) }
    }
    */
    
    /*
    pub fn SDL_SetWindowData(window: *SDL_Window, name: *c_char, userdata: *c_void) -> *c_void; //TODO: Figure out what this does
    pub fn SDL_GetWindowData(window: *SDL_Window, name: *c_char) -> *c_void;
    */

    pub fn set_position(&self, x: WindowPos, y: WindowPos) {
        unsafe { ffi::SDL_SetWindowPosition(self.raw, unwrap_windowpos(x), unwrap_windowpos(y)) }
    }

    pub fn get_position(&self) -> (int, int) {
        let x: c_int = 0;
        let y: c_int = 0;
        unsafe { ffi::SDL_GetWindowPosition(self.raw, &x, &y) };
        (x as int, y as int)
    }

    pub fn set_size(&self, w: int, h: int) {
        unsafe { ffi::SDL_SetWindowSize(self.raw, w as c_int, h as c_int) }
    }

    pub fn get_size(&self) -> (int, int) {
        let w: c_int = 0;
        let h: c_int = 0;
        unsafe { ffi::SDL_GetWindowSize(self.raw, &w, &h) };
        (w as int, h as int)
    }

    pub fn set_minimum_size(&self, w: int, h: int) {
        unsafe { ffi::SDL_SetWindowMinimumSize(self.raw, w as c_int, h as c_int) }
    }

    pub fn get_minimum_size(&self) -> (int, int) {
        let w: c_int = 0;
        let h: c_int = 0;
        unsafe { ffi::SDL_GetWindowMinimumSize(self.raw, &w, &h) };
        (w as int, h as int)
    }

    pub fn set_maximum_size(&self, w: int, h: int) {
        unsafe { ffi::SDL_SetWindowMaximumSize(self.raw, w as c_int, h as c_int) }
    }

    pub fn get_maximum_size(&self) -> (int, int) {
        let w: c_int = 0;
        let h: c_int = 0;
        unsafe { ffi::SDL_GetWindowMaximumSize(self.raw, &w, &h) };
        (w as int, h as int)
    }

    pub fn set_bordered(&self, bordered: bool) {
        unsafe { ffi::SDL_SetWindowBordered(self.raw, if bordered { 1 } else { 0 }) }
    }

    pub fn show(&self) {
        unsafe { ffi::SDL_ShowWindow(self.raw) }
    }

    pub fn hide(&self) {
        unsafe { ffi::SDL_HideWindow(self.raw) }
    }

    pub fn raise(&self) {
        unsafe { ffi::SDL_RaiseWindow(self.raw) }
    }

    pub fn maximize(&self) {
        unsafe { ffi::SDL_MaximizeWindow(self.raw) }
    }

    pub fn minimize(&self) {
        unsafe { ffi::SDL_MinimizeWindow(self.raw) }
    }

    pub fn restore(&self) {
        unsafe { ffi::SDL_RestoreWindow(self.raw) }
    }

    pub fn set_fullscreen(&self, fullscreen_type: FullscreenType) -> bool {
        unsafe { ffi::SDL_SetWindowFullscreen(self.raw, fullscreen_type as uint32_t) == 0 }
    }

    // TODO: implement
    /*
    pub fn get_surface(&self) -> Result<~Surface, ~str> {
        let raw = unsafe { ffi::SDL_GetWindowSurface(self.raw) };

        if raw == ptr::null() {
            Err(get_error())
        } else {
            Ok(~Surface {raw: raw, owned: false}) //Docs say that it releases with the window
        }
    }
    */

    pub fn update_surface(&self) -> bool {
        unsafe { ffi::SDL_UpdateWindowSurface(self.raw) == 0 }
    }

    pub fn update_surface_rects(&self, rects: &[rect::Rect]) -> bool {
        unsafe { ffi::SDL_UpdateWindowSurfaceRects(self.raw, cast::transmute(vec::raw::to_ptr(rects)), rects.len() as c_int) == 0}
    }

    pub fn set_grab(&self, grabbed: bool) {
        unsafe { ffi::SDL_SetWindowGrab(self.raw, if grabbed { 1 } else { 0 }) }
    }

    pub fn get_grab(&self) -> bool {
        unsafe { ffi::SDL_GetWindowGrab(self.raw) == 1 }
    }

    pub fn set_brightness(&self, brightness: f64) -> bool {
        unsafe { ffi::SDL_SetWindowBrightness(self.raw, brightness as c_float) == 0 }
    }

    pub fn get_brightness(&self) -> f64 {
        unsafe { ffi::SDL_GetWindowBrightness(self.raw) as f64 }
    }

    pub fn set_gamma_ramp(&self, red: Option<&[u16, ..256]>, green: Option<&[u16, ..256]>, blue: Option<&[u16, ..256]>) -> bool {
        unsafe {
            let unwrapped_red = match red {
                Some(values) => cast::transmute(vec::raw::to_ptr(*values)),
                None => ptr::null()
            };
            let unwrapped_green = match green {
                Some(values) => cast::transmute(vec::raw::to_ptr(*values)),
                None => ptr::null()
            };
            let unwrapped_blue = match blue {
                Some(values) => cast::transmute(vec::raw::to_ptr(*values)),
                None => ptr::null()
            };
            ffi::SDL_SetWindowGammaRamp(self.raw, unwrapped_red, unwrapped_green, unwrapped_blue) == 0
        }
    }

    pub fn get_gamma_ramp(&self) -> Result<(~[u16], ~[u16], ~[u16]), ~str> {
        let red: ~[u16] = vec::with_capacity(256);
        let green: ~[u16] = vec::with_capacity(256);
        let blue: ~[u16] = vec::with_capacity(256);
        let result = unsafe {ffi::SDL_GetWindowGammaRamp(self.raw, cast::transmute(vec::raw::to_ptr(red)), cast::transmute(vec::raw::to_ptr(green)), cast::transmute(vec::raw::to_ptr(blue))) == 0};
        if result {
            Ok((red, green, blue))
        } else {
            Err(get_error())
        }
    }

    pub fn gl_create_context(&self) -> Result<~GLContext, ~str> {
        let result = unsafe { ffi::SDL_GL_CreateContext(self.raw) };
        if result == ptr::null() {
            Err(get_error())
        } else {
            Ok(~GLContext{raw: result, owned: true})
        }
    }

    pub fn gl_make_current(&self, context: &GLContext) -> bool {
        unsafe { ffi::SDL_GL_MakeCurrent(self.raw, context.raw) == 0 }
    }

    pub fn gl_swap_window(&self) {
        unsafe { ffi::SDL_GL_SwapWindow(self.raw) }
    }
}

pub fn get_num_video_drivers() -> Result<int, ~str> {
    let result = unsafe { ffi::SDL_GetNumVideoDrivers() };
    if result < 0 {
        Err(get_error())
    } else {
        Ok(result as int)
    }
}

pub fn get_video_driver(id: int) -> ~str {
    unsafe {
        let cstr = ffi::SDL_GetVideoDriver(id as c_int);
        str::raw::from_c_str(cast::transmute_copy(&cstr))
    }
}

pub fn video_init(name: &str) -> bool {
    do name.with_c_str |buf| {
        unsafe { ffi::SDL_VideoInit(buf) == 0 }
    }
}

pub fn video_quit() {
    unsafe { ffi::SDL_VideoQuit() }
}

pub fn get_current_video_driver() -> ~str {
    unsafe {
        let cstr = ffi::SDL_GetCurrentVideoDriver();
        str::raw::from_c_str(cast::transmute_copy(&cstr))
    }
}

pub fn get_num_video_displays() -> Result<int, ~str> {
    let result = unsafe { ffi::SDL_GetNumVideoDisplays() };
    if result < 0 {
        Err(get_error())
    } else {
        Ok(result as int)
    }
}

pub fn get_display_name(display_index: int) -> ~str {
    unsafe {
        let cstr = ffi::SDL_GetDisplayName(display_index as c_int);
        str::raw::from_c_str(cast::transmute_copy(&cstr))
    }
}

pub fn get_display_bounds(display_index: int) -> Result<rect::Rect, ~str> {
    let out: rect::Rect = rect::Rect::new(0, 0, 0, 0);
    let result = unsafe { ffi::SDL_GetDisplayBounds(display_index as c_int, &out) == 0 };

    if result {
        Ok(out)
    } else {
        Err(get_error())
    }
}

pub fn get_num_display_modes(display_index: int) -> Result<int, ~str> {
    let result = unsafe { ffi::SDL_GetNumDisplayModes(display_index as c_int) };
    if result < 0 {
        Err(get_error())
    } else {
        Ok(result as int)
    }
}

pub fn get_display_mode(display_index: int, mode_index: int) -> Result<~DisplayMode, ~str> {
    let dm = empty_sdl_display_mode();
    let result = unsafe { ffi::SDL_GetDisplayMode(display_index as c_int, mode_index as c_int, &dm) == 0};

    if result {
        Ok(~DisplayMode::from_ll(&dm))
    } else {
        Err(get_error())
    }
}

pub fn get_desktop_display_mode(display_index: int) -> Result<~DisplayMode, ~str> {
    let dm = empty_sdl_display_mode();
    let result = unsafe { ffi::SDL_GetDesktopDisplayMode(display_index as c_int, &dm) == 0};

    if result {
        Ok(~DisplayMode::from_ll(&dm))
    } else {
        Err(get_error())
    }
}

pub fn get_current_display_mode(display_index: int) -> Result<~DisplayMode, ~str> {
    let dm = empty_sdl_display_mode();
    let result = unsafe { ffi::SDL_GetCurrentDisplayMode(display_index as c_int, &dm) == 0};

    if result {
        Ok(~DisplayMode::from_ll(&dm))
    } else {
        Err(get_error())
    }
}

pub fn get_closest_display_mode(display_index: int, mode: &DisplayMode) -> Result<~DisplayMode, ~str> {
    let input = mode.to_ll();
    let out = empty_sdl_display_mode();

    let result = unsafe { ffi::SDL_GetClosestDisplayMode(display_index as c_int, &input, &out) };

    if result == ptr::null() {
        Err(get_error())
    } else {
        Ok(~DisplayMode::from_ll(&out))
    }
}

pub fn is_screen_saver_enabled() -> bool {
    unsafe { ffi::SDL_IsScreenSaverEnabled() == 1 }
}

pub fn enable_screen_saver() {
    unsafe { ffi::SDL_EnableScreenSaver() }
}

pub fn disable_screen_saver() {
    unsafe { ffi::SDL_DisableScreenSaver() }
}

pub fn gl_load_library(path: &str) -> Result<(), ~str> {
    unsafe {
        do path.with_c_str |path| {
            if ffi::SDL_GL_LoadLibrary(path) == 0 {
                Ok(())
            } else {
                Err(get_error())
            }
        }
    }
}

pub fn gl_unload_library() {
    unsafe { ffi::SDL_GL_UnloadLibrary(); }
}

pub fn gl_get_proc_address(procname: &str) -> Option<extern "C" fn()> {
    unsafe {
        do procname.with_c_str |procname| {
            ffi::SDL_GL_GetProcAddress(procname)
        }
    }
}

pub fn gl_extension_supported(extension: &str) -> bool {
    do extension.with_c_str |buff| {
        unsafe { ffi::SDL_GL_ExtensionSupported(buff) == 1 }
    }
}

pub fn gl_set_attribute(attr: GLAttr, value: int) -> bool {
    unsafe { ffi::SDL_GL_SetAttribute(cast::transmute(attr as u64), value as c_int) == 0 }
}

pub fn gl_get_attribute(attr: GLAttr) -> Result<int, ~str> {
    let out: c_int = 0;

    let result = unsafe { ffi::SDL_GL_GetAttribute(cast::transmute(attr as u64), &out) } == 0;
    if result {
        Ok(out as int)
    } else {
        Err(get_error())
    }
}

pub fn gl_get_current_window() -> Result<~Window, ~str> {
    let raw = unsafe { ffi::SDL_GL_GetCurrentWindow() };
    if raw == ptr::null() {
        Err(get_error())
    } else {
        Ok(~Window{ raw: raw, owned: false })
    }
}

pub fn gl_get_current_context() -> Result<~GLContext, ~str> {
    let raw = unsafe { ffi::SDL_GL_GetCurrentContext() };
    if raw == ptr::null() {
        Err(get_error())
    } else {
        Ok(~GLContext{ raw: raw, owned: false })
    }
}

pub fn gl_set_swap_interval(interval: int) -> bool {
    unsafe { ffi::SDL_GL_SetSwapInterval(interval as c_int) == 0 }
}

pub fn gl_get_swap_interval() -> int {
    unsafe { ffi::SDL_GL_GetSwapInterval() as int }
}
