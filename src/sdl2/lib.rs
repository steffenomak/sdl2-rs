#[pkgid = "sdl2#0.0.1"];
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
pub use rwops::*;
pub use blend_mode::*;

mod linkage {
    #[link(name = "SDL2")]
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
pub mod rwops;
pub mod blend_mode;
