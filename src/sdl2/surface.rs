use std::ptr;
use rect::Rect;

pub mod ffi {
    use pixel::ffi::SDL_PixelFormat;
    use std::libc::{c_int, c_void};
    use rect::Rect;

    pub struct SDL_BlitMap;

    pub struct SDL_Surface {
        format: *SDL_PixelFormat,
        x: c_int,
        y: c_int,
        pitch: c_int,
        pixels: *c_void,
        userdata: *c_void,
        locked: c_int,
        locked_data: *c_void,
        clip_rect: Rect,
        map: *SDL_BlitMap,
        refcount: c_int,
    }

    externfn!(fn BlitScaled(src: *SDL_Surface, 
                            srcrect: *Rect,
                            dst: *SDL_Surface,
                            dstrect: *Rect) -> c_int)
    externfn!(fn SDL_BlitSurface(src: *SDL_Surface, 
                                 srcrect: *Rect,
                                 dst: *SDL_Surface,
                                 destrect: *Rect) -> c_int)
}

pub struct Surface {
    raw: *ffi::SDL_Surface,
    owned: bool,
}

impl Surface {
    fn blit_scaled(&self, 
                   dest_rect: Option<Rect>,
                   src: &Surface,
                   src_rect: Option<Rect>) -> bool {
        unsafe {
            ffi::BlitScaled(src.raw, 
                            if src_rect.is_none() {
                                ptr::null() 
                            } else {
                                ptr::to_unsafe_ptr(&src_rect.unwrap())
                            },
                            self.raw,
                            if dest_rect.is_none() {
                                ptr::null() 
                            } else {
                                ptr::to_unsafe_ptr(&dest_rect.unwrap())
                            }) == 0
        }
    }

    fn blit_surface(&self, 
                    dest_rect: Option<Rect>,
                    src: &Surface,
                    src_rect: Option<Rect>) -> bool {
        unsafe {
            ffi::SDL_BlitSurface(src.raw, 
                                 if src_rect.is_none() {
                                     ptr::null() 
                                 } else {
                                     ptr::to_unsafe_ptr(&src_rect.unwrap())
                                 },
                                 self.raw,
                                 if dest_rect.is_none() {
                                     ptr::null() 
                                 } else {
                                     ptr::to_unsafe_ptr(&dest_rect.unwrap())
                                 }) == 0
        }
    }
}
