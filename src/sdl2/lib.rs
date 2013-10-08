#[link(name = "sdl2", vers = "0.0.2")];
#[crate_type = "lib"];
#[feature(globs)];

pub use error::*;
pub use init::*;
pub use window::*;

mod linkage {
    #[link_args="-lSDL2"]
    extern {}
}

pub mod error;
pub mod init;
pub mod rect;
