mod vid;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;

pub fn main() {
    let mut sdl2sys = vid::Sdl2Sys::new();

    sdl2sys.set_draw_color(Color::RGB(255, 255, 0));
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

        sdl2sys.show_draw_color();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

}
