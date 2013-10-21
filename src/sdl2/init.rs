pub mod ffi {
    use std::libc::{uint32_t, c_int};
    pub type SDL_InitFlag = uint32_t;

    externfn!(fn SDL_Init(flags: SDL_InitFlag) -> c_int)
    externfn!(fn SDL_InitSubSystem(flags: SDL_InitFlag) -> c_int)
    externfn!(fn SDL_Quit())
    externfn!(fn SDL_QuitSubSystem(flags: SDL_InitFlag))
    externfn!(fn SDL_WasInit(flags: SDL_InitFlag) -> SDL_InitFlag)
}

pub enum InitFlag {
    InitTimer          = 0x00000001,
    InitAudio          = 0x00000010,
    InitVideo          = 0x00000020,
    InitJoystick       = 0x00000200,
    InitHaptic         = 0x00001000,
    InitGameController = 0x00002000,
    InitEvents         = 0x00004000,
    InitNoParachute    = 0x00100000,
    InitEverything     = 0x0000FFFF,
}

pub fn init(flags: &[InitFlag]) -> bool {
    unsafe {
        ffi::SDL_Init(do flags.iter().fold(0) |flags, &flag| {
            flags | flag as ffi::SDL_InitFlag
        }) == 0
    }
}

pub fn init_subsystem(flags: &[InitFlag]) -> bool {
    unsafe {
        ffi::SDL_InitSubSystem(do flags.iter().fold(0) |flags, &flag| {
            flags | flag as ffi::SDL_InitFlag
        }) == 0
    }
}

pub fn quit() {
    unsafe {
        ffi::SDL_Quit();
    }
}

pub fn quit_subsystem(flags: &[InitFlag]) {
    unsafe {
        ffi::SDL_QuitSubSystem(do flags.iter().fold(0) |res, &f| {
            res | f as ffi::SDL_InitFlag
        });
    }
}

pub fn was_init(flags: &[InitFlag]) -> ~[InitFlag] {
    let i: ffi::SDL_InitFlag = unsafe {
        ffi::SDL_WasInit(do flags.iter().fold(0) |res, &f| {
            res | f as ffi::SDL_InitFlag
        })
    };

    let flags = [
        InitTimer,
        InitAudio,
        InitVideo,
        InitJoystick,
        InitHaptic,
        InitGameController,
        InitEvents,
        InitNoParachute,
        InitEverything];

    do flags.iter().filter_map |&f| {
        if i & f as ffi::SDL_InitFlag != 0 {
            Some(f)
        } else {
            None
        }
    }.collect()
}
