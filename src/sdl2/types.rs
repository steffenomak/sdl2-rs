pub mod ffi {
    use std::libc::c_int;

    pub type SDL_bool = c_int;
    pub static SDL_TRUE: SDL_bool = 1;
    pub static SDL_FALSE: SDL_bool = 0;
}
