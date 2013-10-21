use std::num;
use video;
use keyboard;

pub mod ffi {
    use std::ptr;
    use std::cast;
    use std::libc::{uint32_t, int32_t, uint8_t, c_int};
    use keyboard::ffi::SDL_Keysym;
    //use super::keyboard::ffi::SDL_Keysym;

    pub enum SDL_EventType {
        SDL_FIRSTEVENT               = 0,
        SDL_QUIT                     = 0x100,

        /* Window events */
        SDL_WINDOWEVENT              = 0x200, /**< Window state change */
        SDL_SYSWMEVENT               = 0x201, /**< System specific event */

        /* Keyboard events */
        SDL_KEYDOWN                  = 0x300, /**< Key pressed */
        SDL_KEYUP                    = 0x301, /**< Key released */
        SDL_TEXTEDITING              = 0x302, /**< Keyboard text editing (composition) */
        SDL_TEXTINPUT                = 0x303, /**< Keyboard text input */

        /* Mouse events */
        SDL_MOUSEMOTION              = 0x400, /**< Mouse moved */
        SDL_MOUSEBUTTONDOWN          = 0x401, /**< Mouse button pressed */
        SDL_MOUSEBUTTONUP            = 0x402, /**< Mouse button released */
        SDL_MOUSEWHEEL               = 0x403, /**< Mouse wheel motion */

        /* Joystick events */
        SDL_JOYAXISMOTION            = 0x600, /**< Joystick axis motion */
        SDL_JOYBALLMOTION            = 0x601, /**< Joystick trackball motion */
        SDL_JOYHATMOTION             = 0x602, /**< Joystick hat position change */
        SDL_JOYBUTTONDOWN            = 0x603, /**< Joystick button pressed */
        SDL_JOYBUTTONUP              = 0x604, /**< Joystick button released */
        SDL_JOYDEVICEADDED           = 0x605, /**< A new joystick has been inserted into the system */
        SDL_JOYDEVICEREMOVED         = 0x606, /**< An opened joystick has been removed */

        /* Game controller events */
        SDL_CONTROLLERAXISMOTION     = 0x650, /**< Game controller axis motion */
        SDL_CONTROLLERBUTTONDOWN     = 0x651, /**< Game controller button pressed */
        SDL_CONTROLLERBUTTONUP       = 0x652, /**< Game controller button released */
        SDL_CONTROLLERDEVICEADDED    = 0x653, /**< A new Game controller has been inserted into the system */
        SDL_CONTROLLERDEVICEREMOVED  = 0x654, /**< An opened Game controller has been removed */
        SDL_CONTROLLERDEVICEREMAPPED = 0x655, /**< The controller mapping was updated */

        /* Touch events */
        SDL_FINGERDOWN               = 0x700,
        SDL_FINGERUP                 = 0x701,
        SDL_FINGERMOTION             = 0x702,

        /* Gesture events */
        SDL_DOLLARGESTURE            = 0x800,
        SDL_DOLLARRECORD             = 0x801,
        SDL_MULTIGESTURE             = 0x802,

        /* Clipboard events */
        SDL_CLIPBOARDUPDATE          = 0x900, /**< The clipboard changed */

        /* Drag and drop events */
        SDL_DROPFILE                 = 0x1000, /**< The system requests a file open */

        /** Events ::SDL_USEREVENT through ::SDL_LASTEVENT are for your use,
         *  and should be allocated with SDL_RegisterEvents()
         */
        SDL_USEREVENT                = 0x8000,

        /**
         *  This last event is only for bounding internal arrays
         */
        SDL_LASTEVENT                = 0xFFFF,
    }

    pub struct SDL_CommonEvent {
        _type: uint32_t,
        time_stamp: uint32_t,
    }

    pub struct SDL_WindowEvent {
        _type: uint32_t,
        time_stamp: uint32_t,
        window_id: uint32_t,
        event: uint8_t,
        padding1: uint8_t,
        padding2: uint8_t,
        padding3: uint8_t,
        data1: int32_t,
        data2: int32_t,
    }

    pub struct SDL_KeyboardEvent {
        _type: uint32_t,
        time_stamp: uint32_t,
        window_id: uint32_t,
        state: uint8_t,
        repeat: uint8_t,
        padding2: uint8_t,
        padding3: uint8_t,
        keysym: SDL_Keysym,
    }

    pub struct SDL_QuitEvent {
        _type: uint32_t,
        time_stamp: uint32_t,
    }

    pub struct SDL_Event {
        event: [uint8_t, ..56],
    }

    impl SDL_Event {
        pub fn new_empty() -> SDL_Event {
            SDL_Event{ event: [0, ..56] }
        }

        pub fn _type(&self) -> *uint32_t {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn common(&self) -> *SDL_CommonEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn window(&self) -> *SDL_WindowEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn keyboard(&self) -> *SDL_KeyboardEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn quit(&self) -> *SDL_QuitEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }
    }
    
    externfn!(fn SDL_PollEvent(event: *SDL_Event) -> c_int)
    externfn!(fn SDL_WaitEvent(event: *SDL_Event) -> c_int)
}

#[deriving(FromPrimitive)]
pub enum EventType {
    FirstEventType = ffi::SDL_FIRSTEVENT as u32,
    QuitEventType = ffi::SDL_QUIT as u32,
    WindowEventType = ffi::SDL_WINDOWEVENT as u32,
    SysWmEventType = ffi::SDL_SYSWMEVENT as u32,
    KeydownEventType = ffi::SDL_KEYDOWN as u32,
    KeyUpEventType = ffi::SDL_KEYUP as u32,
    TextEditingEventType = ffi::SDL_TEXTEDITING as u32,
    TextInputEventType = ffi::SDL_TEXTINPUT as u32,
    MouseMotionEventType = ffi::SDL_MOUSEMOTION as u32,
    MouseButtondownEventType = ffi::SDL_MOUSEBUTTONDOWN as u32,
    MouseButtonupEventType = ffi::SDL_MOUSEBUTTONUP as u32,
    MouseWheelEventType = ffi::SDL_MOUSEWHEEL as u32,
    JoyAxisMotionEventType = ffi::SDL_JOYAXISMOTION as u32,
    JoyBallMotionEventType = ffi::SDL_JOYBALLMOTION as u32,
    JoyHatMotionEventType = ffi::SDL_JOYHATMOTION as u32,
    JoyButtonDownEventType = ffi::SDL_JOYBUTTONDOWN as u32,
    JoyButtonUpEventType = ffi::SDL_JOYBUTTONUP as u32,
    JoyDeviceAddedEventType = ffi::SDL_JOYDEVICEADDED as u32,
    JoyDeviceRemovedEventType = ffi::SDL_JOYDEVICEREMOVED as u32,
    ControllerAxisMotionEventType = ffi::SDL_CONTROLLERAXISMOTION as u32,
    ControllerButtonDownEventType = ffi::SDL_CONTROLLERBUTTONDOWN as u32,
    ControllerButtonUpEventType = ffi::SDL_CONTROLLERBUTTONUP as u32,
    ControllerDeviceAddedEventType = ffi::SDL_CONTROLLERDEVICEADDED as u32,
    ControllerDevicereMovedEventType = ffi::SDL_CONTROLLERDEVICEREMOVED as u32,
    ControllerDevicereMappedEventType = ffi::SDL_CONTROLLERDEVICEREMAPPED as u32,
    FingerDownEventType = ffi::SDL_FINGERDOWN as u32,
    FingerUpEventType = ffi::SDL_FINGERUP as u32,
    FingerMotionEventType = ffi::SDL_FINGERMOTION as u32,
    DollarGestureEventType = ffi::SDL_DOLLARGESTURE as u32,
    DollarRecordEventType = ffi::SDL_DOLLARRECORD as u32,
    MultiGestureEventType = ffi::SDL_MULTIGESTURE as u32,
    ClipboardUpdateEventType = ffi::SDL_CLIPBOARDUPDATE as u32,
    DropFileEventType = ffi::SDL_DROPFILE as u32,
    UserEventType = ffi::SDL_USEREVENT as u32,
    LastEventType = ffi::SDL_LASTEVENT as u32, 
}

pub enum Event {
    NoEvent,
    CommonEvent(u32),
    WindowEvent(u32, ~video::Window, video::WindowEventID, i32, i32),
    /*KeyboardDownEvent(u32, ~video::Window, bool, ~keyboard::Keysym),
    KeyboardUpEvent(u32, ~video::Window, bool, ~keyboard::Keysym),*/
    QuitEvent(u32),
    UnhandeledEvent(u32),
}

impl Event {
    fn wrap_event(raw: &ffi::SDL_Event) -> Event {
        unsafe {
            let raw_type = raw._type();
            let raw_type =
                if raw_type.is_null() {
                    return NoEvent;
                } else {
                    *raw_type
                };

            let event_type: EventType = match num::from_u32(raw_type) {
                Some(n) => n,
                None    => return NoEvent,
            };
            
            match event_type {
                QuitEventType => {
                    let event = raw.quit();

                    let event =
                        if event.is_null() {
                            return NoEvent;
                        } else {
                            *event
                        };

                    QuitEvent(event.time_stamp)
                }
/*                WindowEventType => {
                    let event = raw.window();

                    let event = 
                        if event.is_null() {
                            return NoEvent; 
                        } else {
                            *event
                        };
                    WindowEvent(event.time_stamp, 
                                video::Window::get_from_id(event.window_id).unwrap(),
                                num::from_u8(event.event).unwrap(),
                                event.data1,
                                event.data2)
                }

                KeydownEventType => {
                    let event = raw.keyboard();
                    let event =
                        if event.is_null() {
                            return NoEvent;
                        } else {
                            *event
                        };
                    KeyboardDownEvent(event.time_stamp, 
                                      video::Window::get_from_id(event.window_id).unwrap(),
                                      event.repeat != 0,
                                      keyboard::Keysym::wrap(&event.keysym))
                }

                KeyUpEventType => {
                    let event = raw.keyboard();
                    let event =
                        if event.is_null() {
                            return NoEvent;
                        } else {
                            *event
                        };
                    KeyboardUpEvent(event.time_stamp, 
                                      video::Window::get_from_id(event.window_id).unwrap(),
                                      event.repeat != 0,
                                      keyboard::Keysym::wrap(&event.keysym))

                }
*/
                _ => {
                    UnhandeledEvent(raw_type)
                }
            }
        }
    }
}

pub fn poll_event() -> Event {
    let raw = ffi::SDL_Event::new_empty();   

    let has_event = unsafe {
        ffi::SDL_PollEvent(&raw) == 1
    };

    if has_event {
        Event::wrap_event(&raw)
    } else {
        NoEvent
    }
}
