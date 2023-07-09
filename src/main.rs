mod vid;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;

pub fn main() {
    let mut sdl2sys = vid::Sdl2Sys::new();

    // let ctx = sdl2::init().unwrap();
    // let vid_subsys = ctx.video().unwrap();

    // let win = vid_subsys.window("Comfy Fire", 800, 600).position_centered().build().unwrap();

    // let mut canvas = win.into_canvas().build().unwrap();

    // canvas.set_draw_color(Color::RGB(0, 255, 255));
    // canvas.clear();
    // canvas.present();
    let mut evt_pump = sdl2sys.get_evt_pump();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        sdl2sys.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.clear();
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
        // canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

}
