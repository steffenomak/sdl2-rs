use error::get_error;
use std::str;
use std::num;

mod types;

pub mod ffi {
    use std::libc::{uint8_t, uint16_t, uint32_t, c_int, c_float, c_char};
    use super::types::*;

    pub struct SDL_Color {
        r: uint8_t,
        g: uint8_t,
        b: uint8_t,
        a: uint8_t,
    }

    pub struct SDL_Palette {
        ncolors: c_int,
        colors: *SDL_Color,
        version: uint32_t,
        refcount: c_int,
    }

    pub struct SDL_PixelFormat {
        format: uint32_t,
        palette: *SDL_Palette,
        BitsPerPixel: uint8_t,
        BytesPerPixel: uint8_t,
        padding: [uint8_t, ..2u],
        Rmask: uint32_t,
        Gmask: uint32_t,
        Bmask: uint32_t,
        Amask: uint32_t,
        Rloss: uint8_t,
        Gloss: uint8_t,
        Bloss: uint8_t,
        Aloss: uint8_t,
        Rshift: uint8_t,
        Gshift: uint8_t,
        Bshift: uint8_t,
        Ashift: uint8_t,
        refcount: c_int,
        next: *SDL_PixelFormat,
    }

    extern {
        pub fn SDL_AllocPalette(ncolors: c_int) -> *SDL_Palette;
        pub fn SDL_AllocFormat(pixel_format: uint32_t) -> *SDL_PixelFormat;

        pub fn SDL_CalculateGammaRamp(gamma: c_float, ramp: *uint16_t);

        pub fn SDL_FreePalette(palette: *SDL_Palette);
        pub fn SDL_FreeFormat(format: *SDL_PixelFormat);

        pub fn SDL_GetPixelFormatName(format: uint32_t) -> *c_char;
        pub fn SDL_GetRGB(pixel: uint32_t, 
                          format: *SDL_PixelFormat,
                          r: *uint8_t,
                          g: *uint8_t,
                          b: *uint8_t);
        pub fn SDL_GetRGBA(pixel: uint32_t, 
                           format: *SDL_PixelFormat,
                           r: *uint8_t,
                           g: *uint8_t,
                           b: *uint8_t,
                           a: *uint8_t);
        pub fn SDL_MapRGB(format: *SDL_PixelFormat, 
                          r: uint8_t,
                          g: uint8_t,
                          b: uint8_t) -> uint32_t;
        pub fn SDL_MapRGBA(format: *SDL_PixelFormat, 
                           r: uint8_t,
                           g: uint8_t,
                           b: uint8_t,
                           a: uint8_t) -> uint32_t;
        pub fn SDL_MasksToPixelFormatEnum(bpp: c_int, 
                                          Rmask: uint32_t,
                                          Gmask: uint32_t,
                                          Bmask: uint32_t,
                                          Amask: uint32_t) -> uint32_t;
        pub fn SDL_PixelFormatEnumToMasks(format: uint32_t, 
                                          bpp: *c_int,
                                          Rmask: *uint32_t,
                                          Gmask: *uint32_t,
                                          Bmask: *uint32_t,
                                          Amask: *uint32_t) -> SDL_bool;
        pub fn SDL_SetPaletteColors(palette: *SDL_Palette, 
                                    colors: *SDL_Color,
                                    firstcolor: c_int,
                                    ncolors: c_int) -> c_int;
        pub fn SDL_SetPixelFormatPalette(format: *SDL_PixelFormat, 
                                         palette: *SDL_Palette) -> c_int;
    }
}

#[deriving(FromPrimitive)]
pub enum PixelFormatFlag {
    Unknown     = 0x0,
    Index1LSB   = 0x11100100,
    Index1MSB   = 0x11200100,
    Index4LSB   = 0x12100400,
    Index4MSB   = 0x12200400,
    Index8      = 0x13000801,
    RGB332      = 0x14110801,
    RGB444      = 0x15120c02,
    RGB555      = 0x15130f02,
    BGR555      = 0x15530f02,
    ARGB4444    = 0x15321002,
    RGBA4444    = 0x15421002,
    ABGR4444    = 0x15721002,
    BGRA4444    = 0x15821002,
    ARGB1555    = 0x15331002,
    RGBA5551    = 0x15441002,
    ABGR1555    = 0x15731002,
    BGRA5551    = 0x15841002,
    RGB565      = 0x15151002,
    BGR565      = 0x15551002,
    RGB24       = 0x17101803,
    BGR24       = 0x17401803,
    RGB888      = 0x16161804,
    RGBX8888    = 0x16261804,
    BGR888      = 0x16561804,
    BGRX8888    = 0x16661804,
    ARGB8888    = 0x16362004,
    RGBA8888    = 0x16462004,
    ABGR8888    = 0x16762004,
    BGRA8888    = 0x16862004,
    ARGB2101010 = 0x16372004,
    YV12        = 0x32315659,
    IYUV        = 0x56555949,
    YUY2        = 0x32595559,
    UYVY        = 0x59565955,
    YVYU        = 0x55595659,
}

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn new(r: u8, 
               g: u8,
               b: u8,
               a: u8) -> Color {
        Color {r: r, g: g, b: b, a: a}
    }
}

pub struct Palette {
    raw: *ffi::SDL_Palette,
    owned: bool,
}

pub struct PixelFormat {
    raw: *ffi::SDL_PixelFormat,
    owned: bool,
}

impl Palette {
    pub fn alloc_new(num_col: i32) -> Result<Palette, ~str> {
        unsafe {
            let raw_pal = ffi::SDL_AllocPalette(num_col);

            if raw_pal.is_null() {
                Err(~"Faild to allocate a new palette")
            } else {
                Ok(Palette { raw: raw_pal, owned: true })
            }
        }
    }

    pub fn set_colors(&self,
                  colors: &[Color],
                  first_color: i32,
                  ncolors: i32) -> bool {
        unsafe {
            ffi::SDL_SetPaletteColors(self.raw, 
                                      colors.as_ptr() as *ffi::SDL_Color,
                                      first_color,
                                      ncolors) == 0
        }
    }
}

impl Drop for Palette {
    fn drop(&mut self) {
        unsafe {
            if self.owned {
                debug!("Owned Palette dropped");
                ffi::SDL_FreePalette(self.raw);
            } else {
                debug!("Palette dropped");
            }
        }
    }
}

impl PixelFormat {
    pub fn alloc_new(format: PixelFormatFlag) -> Result<PixelFormat, ~str> {
        unsafe {
            let raw_pix = ffi::SDL_AllocFormat(format as u32);

            if raw_pix.is_null() {
                Err(get_error())
            } else {
                Ok(PixelFormat { raw: raw_pix, owned: true })
            }
        }
    }

    pub fn get_rgb(&self, pixel: u32) -> Color {
        let c = Color {r: 0, g: 0, b: 0, a:0};

        unsafe {
            ffi::SDL_GetRGB(pixel, self.raw, 
                            &(c.r), &(c.g), &(c.b));
        }
        c
    }

    pub fn get_rgba(&self, pixel: u32) -> Color {
        let c = Color {r: 0, g: 0, b: 0, a:0};

        unsafe {
            ffi::SDL_GetRGBA(pixel, self.raw, 
                             &(c.r), &(c.g), &(c.b), &(c.a));
        }
        c
    }

    pub fn map_rgb(&self, c: Color) -> u32 {
        unsafe {
            ffi::SDL_MapRGB(self.raw, c.r, c.g, c.b)
        }
    }

    pub fn map_rgba(&self, c: Color) -> u32 {
        unsafe {
            ffi::SDL_MapRGBA(self.raw, c.r, c.g, c.b, c.a)
        }
    }

    pub fn set_palette(&self, palette: &Palette) -> bool {
        unsafe {
            ffi::SDL_SetPixelFormatPalette(self.raw, palette.raw) == 0
        }
    }
}

impl Drop for PixelFormat {
    fn drop(&mut self) {
        unsafe {
            if self.owned {
                debug!("Owned PixelFormat dropped");
                ffi::SDL_FreeFormat(self.raw);
            } else {
                debug!("PixelFormat dropped");
            }
        }
    }
}

pub fn calculate_gamma_ramp(gamma: f32) -> ~[u16] {
    let tmp = ~[0, ..256];

    unsafe {
        ffi::SDL_CalculateGammaRamp(gamma, tmp.as_ptr());
    }

    tmp
}

pub fn get_pixel_format_name(format: PixelFormatFlag) -> ~str {
    unsafe {
        str::raw::from_c_str(ffi::SDL_GetPixelFormatName(format as u32))
    }
}

pub fn masks_to_pixel_format_flag(bpp: i32, 
                                  r_mask: u32,
                                  g_mask: u32,
                                  b_mask: u32,
                                  a_mask: u32) -> PixelFormatFlag {
    let format = unsafe {
        ffi::SDL_MasksToPixelFormatEnum(bpp, 
                                        r_mask,
                                        g_mask,
                                        b_mask,
                                        a_mask)
    };

    match num::from_u32(format) {
        Some(a) => a,
        None => Unknown
    }
}

pub fn pixel_format_flag_to_mask(format: PixelFormatFlag) -> Option<(i32, 
                                                                     u32,
                                                                     u32,
                                                                     u32,
                                                                     u32)> {
    let r_m = 0;
    let g_m = 0;
    let b_m = 0;
    let a_m = 0;
    let bpp = 0;

    unsafe {
        if ffi::SDL_PixelFormatEnumToMasks(format as u32, 
                                           &bpp,
                                           &r_m, 
                                           &g_m,
                                           &b_m,
                                           &a_m) == types::SDL_TRUE {
            Some((bpp, r_m, g_m, b_m, a_m))
        } else {
            None
        }
    }
}

                                           

