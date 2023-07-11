extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Point;

pub struct Sdl2Sys {
	ctx: sdl2::Sdl,
	// vid: sdl2::VideoSubsystem,
	// win: sdl2::video::Window,
	canvas: sdl2::render::WindowCanvas
}

impl Sdl2Sys {
	pub fn new() -> Sdl2Sys {
		// Constants for window sizing
		const HEIGHT: u32 = 600;
		const WIDTH: u32 = 800;

		// Set up SDL Context, VideoSubsystem, Window, and Canvas
		let ctx: sdl2::Sdl = sdl2::init().unwrap();
		let vid: sdl2::VideoSubsystem = ctx.video().unwrap();
		let win: sdl2::video::Window = vid.window("Comfy Fire", WIDTH, HEIGHT).position_centered().build().unwrap();
		let mut canvas: sdl2::render::WindowCanvas = win.into_canvas().build().unwrap();

		// Initialize blank screen
		canvas.set_draw_color(Color::RGB(0, 0, 0));
		canvas.clear();
		canvas.present();

		// Instantiate and return Sdl2Sys struct
		Sdl2Sys {
			ctx: ctx,
			// vid: vid,
			// win: win,
			canvas: canvas
		}
	}

	pub fn init_fire(&mut self) {
		// Initialize fire image
		let mut g_val: u8 = 255;
		let mut i = 0;
		// I feel like this loop is pretty unoptimized, so I should probably fix it once I learn to git gud at Rust
		while g_val > 0 {
			self.canvas.set_draw_color(Color::RGB(255, g_val, 0));
			while i < self.canvas.window().size().0.try_into().unwrap() {
				let _ = self.canvas.draw_point(Point::new(i, TryInto::<i32>::try_into(self.canvas.window().size().1).unwrap() - 1 - i32::from(255 - g_val)));
				i = i + 4;
			}
			g_val = g_val - 1;
			i = 0;
		}
		self.canvas.present();
	}

	pub fn get_evt_pump(&self) -> sdl2::EventPump {
		self.ctx.event_pump().unwrap()
	}
}
