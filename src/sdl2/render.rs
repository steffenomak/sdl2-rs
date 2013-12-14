use video::{Window, WindowFlags};
use video::ffi::SDL_Window;
use error::get_error;
use surface::Surface;
use pixel::{PixelFormatFlag, Color};
use std::ptr;
use blend_mode::BlendMode;

pub enum RendererFlags {
    Software      = 0x00000001u,
    Accelerated   = 0x00000002u,
    PresentVSync  = 0x00000004u,
    TargetTexture = 0x00000008u,
}

pub enum TextureAccess {
    TextureAccessStatic     = 0,
    TextureAccessStreaming  = 1,
    TextureAccessTarget     = 2,
}

pub enum TextureModulate {
    TextureModulateNone  = 0x00000000u,
    TextureModulateColor = 0x00000001u,
    TextureModulateAlpha = 0x00000002u,
}

pub enum RendererFlip {
    RendererFlipNone       = 0x00000000u,
    RendererFlipHorizontal = 0x00000001u,
    RendererFlipVertical   = 0x00000002u,
}

pub mod ffi {
    use std::libc::{c_int, uint8_t, uint32_t};
    use video::ffi::SDL_Window;
    use surface::ffi::SDL_Surface;
    use blend_mode::BlendMode;

    pub struct SDL_Renderer;
    pub struct SDL_Texture;
    
    extern {
        pub fn SDL_CreateRenderer(window: *SDL_Window, 
                                  index: c_int, 
                                  flags: uint32_t) -> *SDL_Renderer;

        pub fn SDL_CreateSoftwareRenderer(surface: *SDL_Surface) -> *SDL_Renderer;
        pub fn SDL_CreateTexture(rend: *SDL_Renderer, 
                                 format: uint32_t,
                                 access: c_int, 
                                 w: c_int,
                                 h: c_int) -> *SDL_Texture;
        pub fn SDL_CreateTextureFromSurface(renderer: *SDL_Renderer, 
                                            surface: *SDL_Surface) -> *SDL_Texture;
        pub fn SDL_CreateWindowAndRenderer(width: c_int, 
                                           height: c_int,
                                           window_flags: uint32_t,
                                           window: **SDL_Window,
                                           renderer: **SDL_Renderer) -> c_int;
        pub fn SDL_DestroyRenderer(renderer: *SDL_Renderer);
        pub fn SDL_DestroyTexture(texture: *SDL_Texture);
        pub fn SDL_GetNumRenderDrivers() -> c_int;
        pub fn SDL_GetRenderDrawBlendMode(renderer: *SDL_Renderer, 
                                          blendMode: *BlendMode) -> c_int;
        pub fn SDL_GetRenderDrawColor(renderer: *SDL_Renderer, 
                                      r: *uint8_t,
                                      g: *uint8_t,
                                      b: *uint8_t,
                                      a: *uint8_t) -> c_int;
    }
}

pub struct Renderer {
    raw: *ffi::SDL_Renderer,
    owned: bool,
}

pub struct Texture {
    raw: *ffi::SDL_Texture,
    owned: bool,
}

impl Drop for Renderer {
    fn drop(&mut self) {
        if self.owned {
            debug!("Owned Renderer dropped");
            unsafe {
                ffi::SDL_DestroyRenderer(self.raw);
            }
        }
    }
}

impl Renderer {
    pub fn create_renderer(window: &Window, 
                           flags: &[RendererFlags]) -> Result<Renderer, ~str> {
        unsafe {
            let flags = flags.iter().fold(0u32, |a, &b| a | b as u32);
            let rend = ffi::SDL_CreateRenderer(window.raw, -1, flags);

            if rend.is_null() {
                Err(get_error())
            } else {
                Ok(Renderer{ raw: rend, owned: true })
            }
        }
    }

    pub fn create_software_renderer(surface: &Surface) -> Result<Renderer, ~str> {
        unsafe {
            let rend = ffi::SDL_CreateSoftwareRenderer(surface.raw);

            if rend.is_null() {
                Err(get_error())
            } else {
                Ok(Renderer{ raw: rend, owned: true })
            }
        }
    }

    pub fn create_texture(&self, 
                          format: PixelFormatFlag, 
                          access: TextureAccess,
                          w: i32,
                          h: i32) -> Result<Texture, ~str> {
        unsafe {
            let tex = ffi::SDL_CreateTexture(self.raw, 
                                             format as u32,
                                             access as i32,
                                             w, h);

            if tex.is_null() {
                Err(get_error())
            } else {
                Ok( Texture{ raw: tex, owned: true } )
            }
        }
    }

    pub fn texture_from_surface(&self, 
                                surface: &Surface) -> Result<Texture, ~str> {
        unsafe {
            let tex = ffi::SDL_CreateTextureFromSurface(self.raw, surface.raw);

            if tex.is_null() {
                Err(get_error())
            } else {
                Ok( Texture{ raw: tex, owned: true } )
            }
        }
    }

    pub fn create_window_and_renderer(w: i32, h: i32, flags: &[WindowFlags]) -> Result<(Window, Renderer), ~str> {
        let f = flags.iter().fold(0, |all, it| all | *it as u32);

        unsafe {
            let win_ptr: *SDL_Window = ptr::null();
            let rend_ptr: *ffi::SDL_Renderer = ptr::null();
            let r = ffi::SDL_CreateWindowAndRenderer(w, h, f, 
                                                     &win_ptr, &rend_ptr);

            if r == -1 {
                Err(get_error())
            } else {
                Ok(( Window{ raw: win_ptr, owned: true }, 
                     Renderer{ raw: rend_ptr, owned: true }))
            }
        }
    }

    pub fn get_num_driver() -> Result<i32, ~str> {
        let res = unsafe {
            ffi::SDL_GetNumRenderDrivers()
        };

        if res < 1 {
            Err(get_error())
        } else {
            Ok(res)
        }
    }

    pub fn get_draw_blend_mode(&self) -> Result<BlendMode, ~str> {
        //TODO fix this.
        /*let blend = BlendModeNone;
        let res = unsafe {
            ffi::SDL_GetRenderDrawBlendMode(self.raw, &blend);
        };

        if res == 0 {
            Ok(blend)
        } else {*/ 
            Err(get_error())
        //}
    }

    pub fn get_draw_color(&self) -> Result<Color, ~str> {
        let col = Color::new(0, 0, 0, 0);
        let res = unsafe {
            ffi::SDL_GetRenderDrawColor(self.raw, 
                                        &(col.r),
                                        &(col.g),
                                        &(col.b),
                                        &(col.a))
        };

        if res == 0 {
            Ok(col)
        } else {
            Err(get_error())
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        if self.owned {
            debug!("Texture dropped");
            unsafe {
                ffi::SDL_DestroyTexture(self.raw);
            }
        }
    }
}

