use std::num;
use video::{Window, WindowEventID};
use keyboard::Keysym;

pub mod ffi {
    use std::ptr;
    use std::cast;
    use std::libc::{uint32_t, int32_t, uint8_t, c_int};
    use keyboard::ffi::SDL_Keysym;

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
    FirstEventType                    = 0,
    QuitEventType                     = 0x100,
    WindowEventType                   = 0x200, 
    SysWmEventType                    = 0x201, 
    KeydownEventType                  = 0x300, 
    KeyUpEventType                    = 0x301, 
    TextEditingEventType              = 0x302, 
    TextInputEventType                = 0x303, 
    MouseMotionEventType              = 0x400, 
    MouseButtondownEventType          = 0x401, 
    MouseButtonupEventType            = 0x402, 
    MouseWheelEventType               = 0x403, 
    JoyAxisMotionEventType            = 0x600, 
    JoyBallMotionEventType            = 0x601, 
    JoyHatMotionEventType             = 0x602, 
    JoyButtonDownEventType            = 0x603, 
    JoyButtonUpEventType              = 0x604, 
    JoyDeviceAddedEventType           = 0x605, 
    JoyDeviceRemovedEventType         = 0x606, 
    ControllerAxisMotionEventType     = 0x650, 
    ControllerButtonDownEventType     = 0x651, 
    ControllerButtonUpEventType       = 0x652, 
    ControllerDeviceAddedEventType    = 0x653, 
    ControllerDevicereMovedEventType  = 0x654, 
    ControllerDevicereMappedEventType = 0x655, 
    FingerDownEventType               = 0x700,
    FingerUpEventType                 = 0x701,
    FingerMotionEventType             = 0x702,
    DollarGestureEventType            = 0x800,
    DollarRecordEventType             = 0x801,
    MultiGestureEventType             = 0x802,
    ClipboardUpdateEventType          = 0x900, 
    DropFileEventType                 = 0x1000,
    UserEventType                     = 0x8000,
    LastEventType                     = 0xFFFF,
}

pub enum Event {
    NoEvent,
    CommonEvent(u32),
    WindowEvent(u32, ~Window, WindowEventID, i32, i32),
    KeyboardDownEvent(u32, ~Window, bool, ~Keysym),
    KeyboardUpEvent(u32, ~Window, bool, ~Keysym),
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
                WindowEventType => {
                    let event = raw.window();

                    let event = 
                        if event.is_null() {
                            return NoEvent; 
                        } else {
                            *event
                        };
                    WindowEvent(event.time_stamp, 
                                Window::get_from_id(event.window_id).unwrap(),
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
                                      Window::get_from_id(event.window_id).unwrap(),
                                      event.repeat != 0,
                                      Keysym::wrap(&event.keysym))
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
                                      Window::get_from_id(event.window_id).unwrap(),
                                      event.repeat != 0,
                                      Keysym::wrap(&event.keysym))

                }

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
