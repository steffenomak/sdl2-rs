extern mod sdl2;

fn main() {
    let w = 640;
    let h = 480;

    if sdl2::init([sdl2::InitVideo]) == false {
        fail!(format!("SDL could not init!, SDL_Error: {:s}\n", 
                      sdl2::get_error()));
    }

    let window = sdl2::Window::new("SDL Tutorial", 0, 0, w, h, [sdl2::Shown]).unwrap();
    let surface = window.get_surface().unwrap();

    surface.fill_rect(sdl2::Rect::new(0, 0, 640, 480), 
                      sdl2::Color::new(0xFF, 0x00, 0xFF, 0xFF));

    window.update_surface();
    sdl2::delay(2000);

    sdl2::quit();
}
