use video::Window;
use error::get_error;

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
    use std::libc::{c_int, uint32_t};
    use video::ffi::SDL_Window;

    pub struct SDL_Renderer;
    pub struct SDL_Texture;
    
    extern {
        pub fn SDL_CreateRenderer(window: *SDL_Window, 
                                  index: c_int, 
                                  flags: uint32_t) -> *SDL_Renderer;
    }
}

pub struct Renderer {
    raw: *ffi::SDL_Renderer,
    owned: bool,
}

impl Renderer {
    pub fn create_renderer(window: &Window, flags: &[RendererFlags]) -> Result<Renderer, ~str> {
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
}

