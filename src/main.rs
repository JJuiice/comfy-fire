mod vid;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;

pub fn main() {
    let mut sdl2sys: vid::Sdl2Sys = vid::Sdl2Sys::new();

    sdl2sys.init_fire();

    let mut evt_pump = sdl2sys.get_evt_pump();
    let mut frame_delay: u8 = 30;
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

        frame_delay -= 1;
        if frame_delay <= 0 {
            sdl2sys.run_anim();
            frame_delay = 30;
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

}
