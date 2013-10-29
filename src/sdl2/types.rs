pub mod ffi {
    use std::libc::c_int;

    type SDL_bool = c_int;
    static SDL_TRUE: SDL_bool = 1;
    static SDL_FALSE: SDL_bool = 0;
}
