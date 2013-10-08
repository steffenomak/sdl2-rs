mod ffi;

pub enum InitFlag {
    InitTimer = ffi::SDL_INIT_TIMER as int,
    InitAudio = ffi::SDL_INIT_AUDIO as int,
    InitVideo = ffi::SDL_INIT_VIDEO as int,
    InitJoystick = ffi::SDL_INIT_JOYSTICK as int,
    InitHaptic = ffi::SDL_INIT_HAPTIC as int,
    InitGameController = ffi::SDL_INIT_GAMECONTROLLER as int,
    InitEvents = ffi::SDL_INIT_EVENTS as int,
    InitNoParachute = ffi::SDL_INIT_NOPARACHUTE as int,
    InitEverything = ffi::SDL_INIT_EVERYTHING as int,
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
