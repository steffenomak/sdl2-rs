use std::ptr;
use rect::Rect;
use pixel::{PixelFormat, PixelFormatFlag, Color, Palette};
use error::get_error;
use rwops::ffi::SDL_RWFromFile;
use std::path::Path;
use blend_mode::{BlendMode, BlendModeNone};

mod types;

pub mod ffi {
    use pixel::ffi::{SDL_PixelFormat, SDL_Palette};
    use std::libc::{c_int, uint8_t, uint32_t, c_void};
    use rect::Rect;
    use super::types::SDL_bool;
    use rwops::ffi::SDL_RWops;
    use blend_mode::BlendMode;

    pub struct SDL_BlitMap;

    pub struct SDL_Surface {
        flags: uint32_t,
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

    extern {
        pub fn SDL_UpperBlit(src: *SDL_Surface, 
                             srcrect: *Rect,
                             dst: *SDL_Surface,
                             destrect: *Rect) -> c_int;
        pub fn SDL_ConvertSurface(src: *SDL_Surface, 
                                  fmt: *SDL_PixelFormat,
                                  flags: uint32_t) -> *SDL_Surface;
        pub fn SDL_ConvertSurfaceFormat(src: *SDL_Surface, 
                                        pixel_format: uint32_t,
                                        flags: uint32_t) -> *SDL_Surface;
        pub fn SDL_CreateRGBSurface(flags: uint32_t, 
                                    width: c_int,
                                    height: c_int,
                                    depth: c_int,
                                    Rmask: uint32_t,
                                    Gmask: uint32_t,
                                    Bmask: uint32_t,
                                    Amask: uint32_t) -> *SDL_Surface;
        pub fn SDL_FillRect(dst: *SDL_Surface, 
                            rect: *Rect, 
                            color: uint32_t) -> c_int;
        pub fn SDL_FillRects(dst: *SDL_Surface, 
                             rects: *Rect,
                             count: c_int,
                             color: uint32_t) -> c_int;
        pub fn SDL_FreeSurface(surface: *SDL_Surface);
        pub fn SDL_LockSurface(surface: *SDL_Surface) -> c_int;
        pub fn SDL_SetColorKey(surface: *SDL_Surface, 
                               flag: c_int,
                               key: uint32_t) -> c_int;
        pub fn SDL_GetColorKey(surface: *SDL_Surface, 
                               key: *uint32_t) -> c_int;
        pub fn SDL_UnlockSurface(surface: *SDL_Surface);
        pub fn SDL_SetSurfaceRLE(surface: *SDL_Surface, flag: c_int) -> c_int;

        pub fn SDL_LoadBMP_RW(src: *SDL_RWops, freesrc: c_int) -> *SDL_Surface;
        pub fn SDL_SaveBMP_RW(surface: *SDL_Surface, 
                              dst: *SDL_RWops,
                              freedst: c_int) -> c_int;

        pub fn SDL_SetClipRect(surface: *SDL_Surface, 
                               rect: *Rect) -> SDL_bool;
        pub fn SDL_GetCliptRect(surface: *SDL_Surface, 
                                rect: *Rect);

        pub fn SDL_GetSurfaceAlphaMod(surface: *SDL_Surface, 
                                      alpha: *uint8_t) -> c_int;

        pub fn SDL_SetSurfaceAlphaMod(surface: *SDL_Surface, 
                                      alpha: uint8_t) -> c_int;

        pub fn SDL_SetSurfaceBlendMode(surface: *SDL_Surface, 
                                       blendMode: BlendMode) -> c_int;

        pub fn SDL_GetSurfaceBlendMode(surface: *SDL_Surface, 
                                       blendMode: *BlendMode) -> c_int;

        pub fn SDL_SetSurfaceColorMod(surface: *SDL_Surface, 
                                      r: uint8_t,
                                      g: uint8_t,
                                      b: uint8_t) -> c_int;

        pub fn SDL_GetSurfaceColorMod(surface: *SDL_Surface, 
                                      r: *uint8_t,
                                      g: *uint8_t,
                                      b: *uint8_t) -> c_int;

        pub fn SDL_SetSurfacePalette(surface: *SDL_Surface, 
                                     palette: *SDL_Palette) -> c_int;
    }
}

pub struct Surface {
    raw: *ffi::SDL_Surface,
    owned: bool,
}

impl Surface {
    pub fn get_pixel_format(&self) -> PixelFormat {
        unsafe {
            PixelFormat{ raw: (*self.raw).format, owned: false }
        }
    }

    pub fn blit_surface(&self, 
                    dest_rect: Option<Rect>,
                    src: &Surface,
                    src_rect: Option<Rect>) -> bool {
        unsafe {
            ffi::SDL_UpperBlit(src.raw, 
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
                               rects.as_ptr(),
                               rects.len() as i32, c) == 0
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

            let set = if set {
                types::SDL_TRUE
            } else {
                types::SDL_FALSE
            };

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

    pub fn load_bmp(path: &Path) -> Result<Surface, ~str> {
        unsafe {
            let p = SDL_RWFromFile(path.to_c_str().unwrap(), 
                                               "rb".to_c_str().unwrap());
            let raw = ffi::SDL_LoadBMP_RW(p, 1);

            if raw.is_null() {
                Err(get_error())
            } else {
                Ok( Surface{ raw: raw, owned: true } )
            }
        }
    }

    pub fn save_bmp(&self, path: &Path) -> bool {
        unsafe {
            let p = SDL_RWFromFile(path.to_c_str().unwrap(), 
                                   "wb".to_c_str().unwrap());
            ffi::SDL_SaveBMP_RW(self.raw, p, 1) == 0
        }
    }

    pub fn set_clip_rect(&self, rect: &Rect) -> bool {
        unsafe {
            if ffi::SDL_SetClipRect(self.raw, rect) == types::SDL_TRUE {
                true
            } else {
                false
            }
        }
    }

    pub fn get_clip_rect(&self) -> Rect {
        let r = Rect::new(0, 0, 0, 0);

        unsafe { ffi::SDL_GetCliptRect(self.raw, &r); }
        r
    }

    pub fn get_alpha_mod(&self) -> Result<u8, ~str> {
        let alpha: u8 = 0;
        let res = unsafe { ffi::SDL_GetSurfaceAlphaMod(self.raw, &alpha) };

        if res == 0 {
            Ok(alpha)
        } else {
            Err(get_error())
        }
    }

    pub fn set_alpha_mod(&self, alpha: u8) -> bool {
        unsafe { ffi::SDL_SetSurfaceAlphaMod(self.raw, alpha) == 0 }
    }

    pub fn set_blend_mode(&self, blend_mode: BlendMode) -> bool {
        unsafe { ffi::SDL_SetSurfaceBlendMode(self.raw, blend_mode) == 0 }
    }

    pub fn get_blend_mode(&self) -> Result<BlendMode, ~str> {
        let blend = BlendModeNone;
        let r = unsafe { ffi::SDL_GetSurfaceBlendMode(self.raw, &blend) == 0 };

        if r {
            Ok(blend)
        } else {
            Err(get_error())
        }
    }

    pub fn set_color_mod(&self, color: Color) -> bool {
        unsafe { ffi::SDL_SetSurfaceColorMod(self.raw, 
                                             color.r,
                                             color.g,
                                             color.b) == 0 }
    }

    pub fn get_color_mod(&self) -> Result<Color, ~str> {
        let color = Color::new(0, 0, 0, 0);
        let suc = unsafe { ffi::SDL_GetSurfaceColorMod(self.raw, 
                                                       &(color.r),
                                                       &(color.g),
                                                       &(color.b)) == 0 };

        if suc {
            Ok(color)
        } else {
            Err(get_error())
        }
    }

    pub fn set_palette(&self, palette: Palette) -> bool {
        unsafe { ffi::SDL_SetSurfacePalette(self.raw, 
                                            palette.raw) == 0 }
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        if self.owned {
            debug!("Owned Surface dropepd");
            unsafe { ffi::SDL_FreeSurface(self.raw); }
        } else {
            debug!("Surface dropped");
        }
    }
}

