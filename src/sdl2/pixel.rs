pub mod ffi {
    use std::libc::{uint8_t, uint32_t, c_int};

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

    externfn!(fn SDL_AllocPalette(ncolors: c_int) -> *SDL_Palette) 
    externfn!(fn SDL_FreePalette(palette: *SDL_Palette))
}

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

pub struct Palette {
    raw: *ffi::SDL_Palette,
}

impl Palette {
    fn alloc_new(num_col: i32) -> Result<Palette, ~str> {
        unsafe {
            let raw_pal = ffi::SDL_AllocPalette(num_col);

            if raw_pal.is_null() {
                Err(~"Faild to allocate a new palette")
            } else {
                Ok(Palette { raw: raw_pal })
            }
        }
    }
}

impl Drop for Palette {
    fn drop(&mut self) {
        unsafe {
            ffi::SDL_FreePalette(self.raw);
        }
    }
}

