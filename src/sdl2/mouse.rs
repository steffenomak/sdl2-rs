#[deriving(FromPrimitive)]
pub enum Mouse {
    LeftButton   = 1,
    MiddleButton = 2,
    RightButton  = 3,
    X1Button     = 4,
    X2Button     = 5,
}

pub enum MouseState {
    LeftButtonState   = 0x1,
    MiddleButtonState = 0x2,
    RightButtonState  = 0x4,
    X1ButtonState     = 0x8,
    X2ButtonState     = 0xF,
}

impl Mouse {
    pub fn wrap_state(state: u32) -> ~[MouseState] {
        let mut ret: ~[MouseState] = ~[];
        let mask = [
            LeftButtonState,
            MiddleButtonState,
            RightButtonState,
            X1ButtonState,
            X2ButtonState,
            ];

        for m in mask.iter() {
            if state & *m as u32 != 0 {
                ret.push(*m);
            }
        }

        ret
    }
}

