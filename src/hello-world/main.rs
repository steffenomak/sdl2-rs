extern mod sdl2;

fn check_key_works(keysym: ~sdl2::Keysym) {
    match keysym.sym {
        sdl2::AKey => println("A was pressed"),
        sdl2::EscapeKey => println("Esc was pressed"),
        _ => println(format!("Other: {:i}", keysym.sym as i32)),
    }
}

fn handle_window_event(ev: sdl2::WindowEventID) {
    match ev {
        sdl2::MovedEvent => println("Moved"),
        sdl2::FocusLostEvent => println("Focus lost"),
        _ => println("Other"),
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
           sdl2::MouseButtonDownEvent(_, _, btn, _, x, y) => {
               print(format!("x: {:i}, y: {:i} ", x, y));
               match btn {
                   sdl2::LeftButton => println("Left Mouse Button"),
                   sdl2::MiddleButton => println("Middle Mouse Button"),
                   sdl2::RightButton => println("Right Mouse Button"),
                   _ => println("Other"),
               }
           },
           sdl2::WindowEvent(_, _, ev, _, _) => handle_window_event(ev),

           _ => {},
       }
   }


    sdl2::quit();
}

