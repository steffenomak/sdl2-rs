mod keyboard;

pub mod ffi {
    use std::libc::{uint32_t, uint8_t, int32_t};
    use super::keyboard::ffi::SDL_Keysym;

    enum SDL_EventType {
        SDL_FIRSTEVENT = 0,
        SDL_QUIT = 0x100,

        /* Window events */
        SDL_WINDOWEVENT    = 0x200, /**< Window state change */
        SDL_SYSWMEVENT,             /**< System specific event */

        /* Keyboard events */
        SDL_KEYDOWN        = 0x300, /**< Key pressed */
        SDL_KEYUP,                  /**< Key released */
        SDL_TEXTEDITING,            /**< Keyboard text editing (composition) */
        SDL_TEXTINPUT,              /**< Keyboard text input */

        /* Mouse events */
        SDL_MOUSEMOTION    = 0x400, /**< Mouse moved */
        SDL_MOUSEBUTTONDOWN,        /**< Mouse button pressed */
        SDL_MOUSEBUTTONUP,          /**< Mouse button released */
        SDL_MOUSEWHEEL,             /**< Mouse wheel motion */

        /* Joystick events */
        SDL_JOYAXISMOTION  = 0x600, /**< Joystick axis motion */
        SDL_JOYBALLMOTION,          /**< Joystick trackball motion */
        SDL_JOYHATMOTION,           /**< Joystick hat position change */
        SDL_JOYBUTTONDOWN,          /**< Joystick button pressed */
        SDL_JOYBUTTONUP,            /**< Joystick button released */
        SDL_JOYDEVICEADDED,         /**< A new joystick has been inserted into the system */
        SDL_JOYDEVICEREMOVED,       /**< An opened joystick has been removed */

        /* Game controller events */
        SDL_CONTROLLERAXISMOTION  = 0x650, /**< Game controller axis motion */
        SDL_CONTROLLERBUTTONDOWN,          /**< Game controller button pressed */
        SDL_CONTROLLERBUTTONUP,            /**< Game controller button released */
        SDL_CONTROLLERDEVICEADDED,         /**< A new Game controller has been inserted into the system */
        SDL_CONTROLLERDEVICEREMOVED,       /**< An opened Game controller has been removed */
        SDL_CONTROLLERDEVICEREMAPPED,      /**< The controller mapping was updated */

        /* Touch events */
        SDL_FINGERDOWN      = 0x700,
        SDL_FINGERUP,
        SDL_FINGERMOTION,

        /* Gesture events */
        SDL_DOLLARGESTURE   = 0x800,
        SDL_DOLLARRECORD,
        SDL_MULTIGESTURE,

        /* Clipboard events */
        SDL_CLIPBOARDUPDATE = 0x900, /**< The clipboard changed */

        /* Drag and drop events */
        SDL_DROPFILE        = 0x1000, /**< The system requests a file open */

        /** Events ::SDL_USEREVENT through ::SDL_LASTEVENT are for your use,
         *  and should be allocated with SDL_RegisterEvents()
         */
        SDL_USEREVENT    = 0x8000,

        /**
         *  This last event is only for bounding internal arrays
         */
        SDL_LASTEVENT    = 0xFFFF
    }

    struct SDL_CommonEvent {
        _type: uint32_t,
        time_stamp: uint32_t,
    }

    struct SDL_WindowEvent {
        _type: uint32_t,
        time_stamp: uint32_t,
        window_id: uint32_t,
        event: uint32_t,
        padding1: uint8_t,
        padding2: uint8_t,
        padding3: uint8_t,
        data1: int32_t,
        data2: int32_t,
    }

    struct SDL_KeyboadEvent {
        _type: uint32_t,
        time_stamp: uint32_t,
        window_id: uint32_t,
        state: uint8_t,
        repeat: uint8_t,
        padding2: uint8_t,
        padding3: uint8_t,
        keysym: SDL_Keysym,
    }

    struct SDL_QuitEvent {
        _type: uint32_t,
        time_stamp: uint32_t,
    }

    struct SDL_Event {
        event: [uint8_t, ..56],
    }
}

