extern crate piston_window;

use piston_window::*;
use std::ops::*;

const ball_speed: f64 = 1.0;

struct Ball {
	x: f64,
	y: f64,
	vx: f64,
	vy: f64,
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
	fn bounces(&self, dx: f64, dy: f64) -> bool {
		self.x + dx > 1.0 || self.x + dx < -1.0 || self.y + dy > 1.0 || self.y + dy < -1.0
	}
	fn update(&self, dt: f64) -> Ball {
		let x2 = self.x + self.vx*dt;
		let y2 = self.y + self.vy*dt;
		let bounce = self.bounces(self.vx*dt, self.vy*dt);
		Ball {
			x: bound_wrap(self.x + self.vx*dt, -1.0, 1.0),
			y: bound_wrap(self.y + self.vy*dt, -1.0, 1.0),
			vx: if bounce { -self.vx } else { self.vx },
			vy: if bounce { -self.vy } else { self.vy },
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
					vx: ball_speed,
					vy: 0.0,
					r: 0.05
				}, Ball {
					x: 0.5,
					y: -0.5,
					vx: 0.0,
					vy: ball_speed,
					r: 0.05
				}
			]
		}
	}
}

fn main() {
	// TODO: width/height consts
	let mut window : PistonWindow = WindowSettings::new("Biobox", [400,400])
		.opengl(OpenGL::V3_2)
		.exit_on_esc(true)
		.vsync(true)
		.build()
		.unwrap(); // TODO: error handling

	window.set_ups(60);

	let mut app = App::init();

	while let Some(e) = window.next() {
		window.draw_2d(&e, |c, g| {
			clear(color::WHITE, g);
			for b in app.balls.into_iter() {
				let x = (b.x/2.0+0.5)*400.0;
				let y = (b.y/2.0+0.5)*400.0;
				// println!("x: {}, y: {}", x, y);
				Ellipse::new([1.0, 0.0, 0.0, 0.5])
					.draw(ellipse::circle(x, y, b.r*400.0), &c.draw_state, c.transform, g);
			}
			app.balls[0] = app.balls[0].update(1.0/60.0);
			app.balls[1] = app.balls[1].update(1.0/60.0);
		});
	}
}
