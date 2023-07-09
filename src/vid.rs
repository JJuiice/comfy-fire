extern crate sdl2;

use sdl2::pixels::Color;

pub struct Sdl2Sys {
	ctx: sdl2::Sdl,
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
		canvas.set_draw_color(Color::RGB(0, 255, 255));
		canvas.clear();
		canvas.present();

		Sdl2Sys {
			ctx: ctx,
			// vid: vid,
			// win: win,
			canvas: canvas
		}
	}

	pub fn set_draw_color(&mut self, color: sdl2::pixels::Color) {
		self.canvas.set_draw_color(color);
		self.canvas.clear();
	}

	pub fn show_draw_color(&mut self) {
		self.canvas.present();
	}

	pub fn get_evt_pump(&self) -> sdl2::EventPump {
		self.ctx.event_pump().unwrap()
	}
}
