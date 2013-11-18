extern mod sdl2;

use std::path::Path;

fn init(w: i32, h: i32) -> sdl2::Window {
    sdl2::init([sdl2::InitVideo]);

    sdl2::Window::new("SDL Tutorial", 0, 0, w, h, [sdl2::Shown]).unwrap()
}

fn load_media(path: &Path, format: &sdl2::PixelFormat) -> sdl2::Surface {
    let tmp = sdl2::Surface::load_bmp(path).unwrap();
    tmp.convert_surface(format).unwrap()
}

fn main() {
    let win = ~init(640, 480);
    let surf = win.get_surface().unwrap();
    let hello = load_media(&Path::new("./bin/img.bmp"), &surf.get_pixel_format());


    surf.blit_surface(None, &hello, None);

    win.update_surface();

    loop {
        match sdl2::poll_event() {
            sdl2::NoEvent => (),
            sdl2::QuitEvent(_) => break,
            sdl2::KeyboardDownEvent(_, _, _, sym) => {
                if sym.sym == sdl2::EscapeKey {
                    break;
                } else if sym.sym == sdl2::UpKey {
                    println("Hello World");
                }
            },
            _ => (),
        }
    }
    sdl2::quit();
}
