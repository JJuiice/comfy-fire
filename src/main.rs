mod vid;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;

pub fn main() {
    let mut sdl2sys: vid::Sdl2Sys = vid::Sdl2Sys::new();

    sdl2sys.init_fire();
    let mut evt_pump = sdl2sys.get_evt_pump();
    'running: loop {
        for evt in evt_pump.poll_iter() {
            match evt {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

}
