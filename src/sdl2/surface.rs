use std::ptr;
use rect::Rect;
use pixel::{PixelFormat, PixelFormatFlag, Color};
use error::get_error;
use std::vec;

pub mod ffi {
    use pixel::ffi::SDL_PixelFormat;
    use std::libc::{c_int, uint32_t, c_void, c_char};
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
    externfn!(fn SDL_ConvertSurface(src: *SDL_Surface, 
                                    fmt: *SDL_PixelFormat,
                                    flags: uint32_t) -> *SDL_Surface)
    externfn!(fn SDL_ConvertSurfaceFormat(src: *SDL_Surface, 
                                          pixel_format: uint32_t,
                                          flags: uint32_t) -> *SDL_Surface)
    externfn!(fn SDL_CreateRGBSurface(flags: uint32_t, 
                                      width: c_int,
                                      height: c_int,
                                      depth: c_int,
                                      Rmask: uint32_t,
                                      Gmask: uint32_t,
                                      Bmask: uint32_t,
                                      Amask: uint32_t) -> *SDL_Surface)
    externfn!(fn SDL_FillRect(dst: *SDL_Surface, 
                              rect: *Rect, 
                              color: uint32_t) -> c_int)
    externfn!(fn SDL_FillRects(dst: *SDL_Surface, 
                               rects: *Rect,
                               count: c_int,
                               color: uint32_t) -> c_int)
    externfn!(fn SDL_FreeSurface(surface: *SDL_Surface))
    externfn!(fn SDL_LoadBMP(file: *c_char) -> *SDL_Surface)
    externfn!(fn SDL_LockSurface(surface: *SDL_Surface) -> c_int)
    externfn!(fn SDL_SaveBMP(surface: *SDL_Surface, file: *c_char) -> c_int)
    externfn!(fn SDL_SetColorKey(surface: *SDL_Surface, 
                                 flag: c_int,
                                 key: uint32_t) -> c_int)
    externfn!(fn SDL_GetColorKey(surface: *SDL_Surface, 
                                 key: *uint32_t) -> c_int)
    externfn!(fn SDL_UnlockSurface(surface: *SDL_Surface))
    externfn!(fn SDL_SetSurfaceRLE(surface: *SDL_Surface, flag: c_int) -> c_int)
}

pub struct Surface {
    raw: *ffi::SDL_Surface,
    owned: bool,
}

impl Surface {
    pub fn blit_scaled(&self, 
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

    pub fn blit_surface(&self, 
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

    pub fn convert_surface(&self, fmt: &PixelFormat) -> Result<Surface, ~str> {
        unsafe {
            let surf = ffi::SDL_ConvertSurface(self.raw, fmt.raw, 0);

            if surf.is_null() {
                Err(get_error())
            } else {
                Ok(Surface{ raw: surf, owned: true })
            }
        }
    }

    pub fn convert_surface_format(&self, 
                              fmt: PixelFormatFlag) -> Result<Surface, ~str> {
        unsafe {
            let surf = ffi::SDL_ConvertSurfaceFormat(self.raw, fmt as u32, 0);

            if surf.is_null() {
                Err(get_error())
            } else {
                Ok(Surface { raw: surf, owned: true })
            }
        }
    }

    pub fn create_rgb_surface(w:      i32,
                          h:      i32,
                          bpp:    i32,
                          r_mask: u32,
                          g_mask: u32,
                          b_mask: u32,
                          a_mask: u32) -> Result<Surface, ~str> {
        unsafe {
            let surf = ffi::SDL_CreateRGBSurface(0, w, h, bpp, 
                                                 r_mask,
                                                 g_mask,
                                                 b_mask,
                                                 a_mask);

            if surf.is_null() {
                Err(get_error())
            } else {
                Ok(Surface{ raw: surf, owned: true })
            }
        }
    }

    pub fn fill_rect(&self, rect: Rect, color: Color) -> bool {
        unsafe {
            let c = PixelFormat{raw: (*self.raw).format, owned: false};
            let c = c.map_rgba(color);

            ffi::SDL_FillRect(self.raw, 
                              ptr::to_unsafe_ptr(&rect),
                              c) == 0
        }
    }
    
    pub fn fill_rects(&self, rects: &[Rect], color: Color) -> bool {
        unsafe {
            let c = PixelFormat{raw: (*self.raw).format, owned: false};
            let c = c.map_rgba(color);

            ffi::SDL_FillRects(self.raw, 
                               vec::raw::to_ptr(rects), 
                               rects.len() as i32, c) == 0
        }
    }

    pub fn load_bmp(file: ~str) -> Result<Surface, ~str> {
        unsafe {
            let surf = do file.with_c_str |buf| {
                ffi::SDL_LoadBMP(buf)
            };

            if surf.is_null() {
                Err(get_error())
            } else {
                Ok(Surface{ raw: surf, owned: true })
            }
        }
    }

    pub fn save_bmp(&self, file: ~str) -> bool {
        unsafe {
            do file.with_c_str |buf| {
                ffi::SDL_SaveBMP(self.raw, buf) == 0
            }
        }
    }

    pub fn lock(&self) -> bool {
        unsafe {
            ffi::SDL_LockSurface(self.raw) == 0
        }
    }

    pub fn unlock(&self) {
        unsafe {
            ffi::SDL_UnlockSurface(self.raw);
        }
    }

    pub fn set_color_key(&self, set: bool, key: Color) -> bool {
        unsafe {
            let c = PixelFormat{raw: (*self.raw).format, owned: false};
            let c = c.map_rgba(key);

            ffi::SDL_SetColorKey(self.raw, set as i32, c) == 0
        }
    }

    pub fn get_color_key(&self) -> Result<Color, ~str> {
        unsafe {
            let color_key = 0u32;
            let c = PixelFormat{raw: (*self.raw).format, owned: false};

            match ffi::SDL_GetColorKey(self.raw, &color_key) {
                0 => Ok(c.get_rgba(color_key)),
                _ => Err(get_error()),
            }
        }
    }

    pub fn set_rle(&self, set: bool) -> bool {
        unsafe {
            ffi::SDL_SetSurfaceRLE(self.raw, set as i32) == 0
        }
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        if self.owned {
            unsafe { ffi::SDL_FreeSurface(self.raw); }
        }
    }
}

