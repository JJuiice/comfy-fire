extern crate sdl2;

use rand::Rng;

use sdl2::pixels::Color;
use sdl2::rect::Point;

// Constants for window sizing
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

pub struct Sdl2Sys {
	ctx: sdl2::Sdl,
	canvas: sdl2::render::WindowCanvas,
	rng: rand::rngs::ThreadRng,
	spark_ps: [Point; 10]
}

impl Sdl2Sys {
	pub fn new() -> Sdl2Sys {
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
			canvas: canvas,
			rng: rand::thread_rng(),
			spark_ps: [Point::new((HEIGHT - (HEIGHT / 4) - 50) as i32, 0); 10]
		}
	}

	pub fn init_fire(&mut self) {
		let mut points: [[Point; 640]; 180] = [[Point::new(0, 0); (WIDTH / 2) as usize]; (HEIGHT / 4) as usize];

		for y in 0..(HEIGHT / 4) {
			for x in 0..(WIDTH / 2) {
				points[y as usize][x as usize] = points[y as usize][x as usize].offset((x * 2) as i32, (HEIGHT - (y * 2)) as i32);
			}

			let grad_factor: f64 = 1.0 - (f64::from(y as u32) + 1.0) / (f64::from(HEIGHT) / 4.0);
			self.canvas.set_draw_color(Color::RGB(255, (255.0 * grad_factor).floor() as u8, 0));
			self.canvas.draw_points(points[y as usize].as_slice()).unwrap();
		}

		self.canvas.present();
	}

	pub fn run_anim(&mut self) {
		self.canvas.set_draw_color(Color::RGB(0, 0, 0));

		let _ = self.canvas.draw_points(self.spark_ps.as_slice()).unwrap();

		for point in &mut self.spark_ps {
			*point = Point::new(self.rng.gen_range(0..WIDTH) as i32, (HEIGHT - (HEIGHT / 2) - 50) as i32);
		}

		self.canvas.set_draw_color(Color::RGB(255, 120, 0));

		let _ = self.canvas.draw_points(self.spark_ps.as_slice()).unwrap();

		self.canvas.present();
	}

	pub fn get_evt_pump(&mut self) -> sdl2::EventPump {
		self.ctx.event_pump().unwrap()
	}
}
