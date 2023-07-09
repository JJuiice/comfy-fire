mod vid;

use std::time::Duration;

pub fn main() {
    // let ctx = sdl2::init().unwrap();
    // let vid_subsys = ctx.video().unwrap();

    // let win = vid_subsys.window("Comfy Fire", 800, 600).position_centered().build().unwrap();

    // let mut canvas = win.into_canvas().build().unwrap();

    // canvas.set_draw_color(Color::RGB(0, 255, 255));
    // canvas.clear();
    // canvas.present();
    // let mut evt_pump = ctx.event_pump().unwrap();
    // let mut i = 0;
    // 'running: loop {
    //     i = (i + 1) % 255;
    //     canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
    //     canvas.clear();
    //     for evt in evt_pump.poll_iter() {
    //         match evt {
    //             Event::Quit {..} |
    //             Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
    //                 break 'running
    //             },
    //             _ => {}
    //         }
    //     }

    //     canvas.present();
    //     ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    // }

    vid::Sdl2Sys::new();

    // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32));
}
