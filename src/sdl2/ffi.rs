use std::libc::{c_char, c_int, uint32_t};
use std::libc::{c_void, c_float, uint16_t};

// error functions
externfn!(fn SDL_ClearError())
externfn!(fn SDL_GetError() -> *c_char)
externfn!(fn SDL_SetError(err: *c_char))

// basic functions
pub type SDL_InitFlag = uint32_t;

pub static SDL_INIT_TIMER: SDL_InitFlag          = 0x00000001;
pub static SDL_INIT_AUDIO: SDL_InitFlag          = 0x00000010;
pub static SDL_INIT_VIDEO: SDL_InitFlag          = 0x00000020;
pub static SDL_INIT_JOYSTICK: SDL_InitFlag       = 0x00000200;
pub static SDL_INIT_HAPTIC: SDL_InitFlag         = 0x00001000;
pub static SDL_INIT_GAMECONTROLLER: SDL_InitFlag = 0x00002000;
pub static SDL_INIT_EVENTS: SDL_InitFlag         = 0x00004000;
pub static SDL_INIT_NOPARACHUTE: SDL_InitFlag    = 0x00100000;
pub static SDL_INIT_EVERYTHING: SDL_InitFlag     = 0x0000FFFF;

externfn!(fn SDL_Init(flags: SDL_InitFlag) -> c_int)
externfn!(fn SDL_InitSubSystem(flags: SDL_InitFlag) -> c_int)
externfn!(fn SDL_Quit())
externfn!(fn SDL_QuitSubSystem(flags: SDL_InitFlag))
externfn!(fn SDL_WasInit(flags: SDL_InitFlag) -> SDL_InitFlag)

// video functions
pub struct SDL_Window;

pub type SDL_WindowPos = c_int;
pub static SDL_WINDOWPOS_CENTERED: SDL_WindowPos = 0x2FFF0000;
pub static SDL_WINDOWPOS_UNDEFINED: SDL_WindowPos = 0x1FFF0000;

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
    SDL_WINDOWEVENT_NONE,
    SDL_WINDOWEVENT_SHOWN,
    SDL_WINDOWEVENT_HIDDEN,
    SDL_WINDOWEVENT_EXPOSED,
    SDL_WINDOWEVENT_MOVED,
    SDL_WINDOWEVENT_RESIZED,
    SDL_WINDOWEVENT_SIZE_CHANGED,
    SDL_WINDOWEVENT_MINIMIZED,
    SDL_WINDOWEVENT_MAXIMIZED,
    SDL_WINDOWEVENT_RESTORED,
    SDL_WINDOWEVENT_ENTER,
    SDL_WINDOWEVENT_LEAVE,
    SDL_WINDOWEVENT_FOCUS_GAINED,
    SDL_WINDOWEVENT_FOCUS_LOST,
    SDL_WINDOWEVENT_CLOSE
}

pub type SDL_GLContext = *c_void;

pub enum SDL_GLattr {
    SDL_GL_RED_SIZE = 0,
    SDL_GL_GREEN_SIZE = 1,
    SDL_GL_BLUE_SIZE = 2,
    SDL_GL_ALPHA_SIZE = 3,
    SDL_GL_BUFFER_SIZE = 4,
    SDL_GL_DOUBLEBUFFER = 5,
    SDL_GL_DEPTH_SIZE = 6,
    SDL_GL_STENCIL_SIZE = 7,
    SDL_GL_ACCUM_RED_SIZE = 8,
    SDL_GL_ACCUM_GREEN_SIZE = 9,
    SDL_GL_ACCUM_BLUE_SIZE = 10,
    SDL_GL_ACCUM_ALPHA_SIZE = 11,
    SDL_GL_STEREO = 12,
    SDL_GL_MULTISAMPLEBUFFERS = 13,
    SDL_GL_MULTISAMPLESAMPLES = 14,
    SDL_GL_ACCELERATED_VISUAL = 15,
    SDL_GL_RETAINED_BACKING = 16,
    SDL_GL_CONTEXT_MAJOR_VERSION = 17,
    SDL_GL_CONTEXT_MINOR_VERSION = 18,
    SDL_GL_CONTEXT_EGL = 19,
    SDL_GL_CONTEXT_FLAGS = 20,
    SDL_GL_CONTEXT_PROFILE_MASK = 21,
    SDL_GL_SHARE_WITH_CURRENT_CONTEXT = 22
}

pub enum SDL_GLprofile {
    SDL_GL_CONTEXT_PROFILE_CORE = 0x0001,
    SDL_GL_CONTEXT_PROFILE_COMPATIBILITY = 0x0002,
    SDL_GL_CONTEXT_PROFILE_ES = 0x0004
}

externfn!(fn SDL_CreateWindow(title: *c_char, x: c_int, y: c_int, w: c_int, h: c_int, flags: uint32_t) -> *SDL_Window)
externfn!(fn SDL_GetWindowID(window: *SDL_Window) -> uint32_t)
externfn!(fn SDL_GetWindowFromID(id: uint32_t) -> *SDL_Window)
externfn!(fn SDL_GetWindowFlags(window: *SDL_Window) -> uint32_t)
externfn!(fn SDL_SetWindowTitle(window: *SDL_Window, title: *c_char))
externfn!(fn SDL_GetWindowTitle(window: *SDL_Window) -> *c_char)
externfn!(fn SDL_SetWindowPosition(window: *SDL_Window, x: c_int, y: c_int))
externfn!(fn SDL_GetWindowPosition(window: *SDL_Window, x: *c_int, y: *c_int))
externfn!(fn SDL_SetWindowSize(window: *SDL_Window, w: c_int, h: c_int))
externfn!(fn SDL_GetWindowSize(window: *SDL_Window, w: *c_int, h: *c_int))
externfn!(fn SDL_SetWindowMinimumSize(window: *SDL_Window, min_w: c_int, min_h: c_int))
externfn!(fn SDL_GetWindowMinimumSize(window: *SDL_Window, w: *c_int, h: *c_int))
externfn!(fn SDL_SetWindowMaximumSize(window: *SDL_Window, max_w: c_int, max_h: c_int))
externfn!(fn SDL_GetWindowMaximumSize(window: *SDL_Window, w: *c_int, h: *c_int))
externfn!(fn SDL_ShowWindow(window: *SDL_Window))
externfn!(fn SDL_HideWindow(window: *SDL_Window))
externfn!(fn SDL_RaiseWindow(window: *SDL_Window))
externfn!(fn SDL_MaximizeWindow(window: *SDL_Window))
externfn!(fn SDL_MinimizeWindow(window: *SDL_Window))
externfn!(fn SDL_RestoreWindow(window: *SDL_Window))
externfn!(fn SDL_SetWindowFullscreen(window: *SDL_Window, flags: uint32_t) -> c_int)
externfn!(fn SDL_UpdateWindowSurface(window: *SDL_Window) -> c_int)
externfn!(fn SDL_DestroyWindow(window: *SDL_Window))
externfn!(fn SDL_GL_GetProcAddress(proc: *c_char) -> Option<extern "C" fn()>)
externfn!(fn SDL_GL_SetAttribute(attr: SDL_GLattr, value: c_int) -> c_int)
externfn!(fn SDL_GL_GetAttribute(attr: SDL_GLattr, value: *c_int) -> c_int)
externfn!(fn SDL_GL_CreateContext(window: *SDL_Window) -> SDL_GLContext)
externfn!(fn SDL_GL_MakeCurrent(window: *SDL_Window, context: SDL_GLContext) -> c_int)
externfn!(fn SDL_GL_GetCurrentWindow() -> *SDL_Window)
externfn!(fn SDL_GL_GetCurrentContext() -> SDL_GLContext)
externfn!(fn SDL_GL_SetSwapInterval(interval: c_int) -> c_int)
externfn!(fn SDL_GL_GetSwapInterval() -> c_int)
externfn!(fn SDL_GL_SwapWindow(window: *SDL_Window))
externfn!(fn SDL_GL_DeleteContext(context: SDL_GLContext))
