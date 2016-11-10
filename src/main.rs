// extern crate piston;
// extern crate graphics;
extern crate piston_window;

use piston_window::*;
use std::ops::*;

const ball_speed: f64 = 1.0;

struct Ball {
	x: f64,
	y: f64,
	dx: f64,
	dy: f64,
	r: f64
}

// Assumes one 'wrap' will always be enough (the result is always within the bounds)
// Apparently generic numerical functions are pretty painful
fn bound_wrap<T: Add<Output=T> + Sub<Output=T> + PartialOrd + Copy>(x: T, min: T, max: T) -> T {
	if x > max {
		max - (x - max)
	} else if x < min {
		min + (min - x)
	} else {
		x
	}
}

impl Ball {
	fn update(&self, dt: f64) -> Ball {
		let x2 = self.x + self.dx*dt;
		let y2 = self.y + self.dy*dt;
		Ball {
			x: bound_wrap(self.x + self.dx*dt, -1.0, 1.0),
			y: bound_wrap(self.y + self.dy*dt, -1.0, 1.0),
			.. *self
		}
	}
}

struct App {
	balls: [Ball; 2]
}

impl App {
	fn init() -> App {
		App {
			balls: [
				Ball {
					x: -1.0,
					y: 0.5,
					dx: ball_speed,
					dy: 0.0,
					r: 0.05
				}, Ball {
					x: 0.5,
					y: -0.5,
					dx: 0.0,
					dy: ball_speed,
					r: 0.05
				}
			]
		}
	}
}

fn main() {
	println!("Hello, world!");

	let mut window : PistonWindow = WindowSettings::new("Biobox", [400,300])
		.opengl(OpenGL::V3_2)
		.exit_on_esc(true)
		.vsync(true)
		.build()
		.unwrap(); // TODO: error handling

	window.set_ups(60);

	let app = App::init();

	while let Some(e) = window.next() {
		window.draw_2d(&e, |c, g| {
			clear(color::WHITE, g);
			for b in app.balls.into_iter() {
				// TODO: draw balls
			}
		});
	}
}
