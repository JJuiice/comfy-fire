extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Sdl2Sys {
	// ctx: sdl2::Sdl,
	// vid: sdl2::VideoSubsystem,
	// win: sdl2::video::Window,
	canvas: sdl2::render::WindowCanvas
}

impl Sdl2Sys {
	pub fn new() -> Sdl2Sys {
		let ctx = sdl2::init().unwrap();
		let vid = ctx.video().unwrap();
		let win = vid.window("Comfy Fire", 800, 600).position_centered().build().unwrap();
		let mut canvas = win.into_canvas().build().unwrap();
		canvas.set_draw_color(Color::RGB(0, 0, 0));
		canvas.clear();
		canvas.present();

		Sdl2Sys {
			// ctx: ctx,
			// vid: vid,
			// win: win,
			canvas: canvas
		}
	}
}
