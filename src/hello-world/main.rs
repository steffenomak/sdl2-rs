extern mod sdl2;

fn main() {
    sdl2::init([sdl2::InitVideo]);

    sdl2::quit();
}
