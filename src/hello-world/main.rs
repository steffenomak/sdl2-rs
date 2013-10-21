extern mod sdl2;

fn check_key_works(keysym: ~sdl2::Keysym) {
    match keysym.sym {
        sdl2::AKey => println("A was pressed"),
        sdl2::EscapeKey => println("Esc was pressed"),
        _ => println(format!("Other: {:i}", keysym.sym as i32)),
    }
}

fn main() {
    sdl2::init([sdl2::InitVideo]);

    let window = 
        match sdl2::Window::new("hello world", 0, 0, 600, 400, [sdl2::OpenGL]) {
            Ok(win) => win,
            Err(msg) => fail!(format!("faild to create window: {:s}", msg))
        };

   loop {
       match sdl2::poll_event() {
           sdl2::QuitEvent(_) => 
           {
               println("Trollololo");
               break;
           },
           sdl2::KeyboardUpEvent(_, _, _, keysym) => check_key_works(keysym),
           sdl2::UnhandeledEvent(ev) => println(format!("Event: {:x}", ev)),
           _ => {},
       }
   }


    sdl2::quit();
}

