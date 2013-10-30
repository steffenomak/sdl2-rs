#[link(name = "sdl2", vers = "0.0.2")];
#[crate_type = "lib"];
#[feature(globs)];

pub use error::*;
pub use init::*;
pub use rect::*;
pub use event::*;
pub use keycode::*;
pub use scancode::*;
pub use video::*;
pub use keyboard::*;
pub use mouse::*;
pub use pixel::*;
pub use surface::*;
pub use render::*;
pub use timer::*;

mod linkage {
    #[link_args="-lSDL2"]
    extern {}
}

pub mod error;
pub mod init;
pub mod rect;
pub mod event;
pub mod keycode;
pub mod scancode;
pub mod video;
pub mod keyboard;
pub mod mouse;
pub mod pixel;
pub mod surface;
pub mod render;
pub mod timer;
