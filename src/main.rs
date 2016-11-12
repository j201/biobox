extern crate piston_window;
extern crate nalgebra as na;
extern crate rand;

use piston_window::*;
use std::ops::*;
use rand::distributions::IndependentSample;
use rand::distributions as rdist;

const MAX_V: f64 = 2.0;

type Vect = na::Vector2<f64>;
type Point = na::Point2<f64>;

struct Ball {
	p: Point,
	v: Vect,
	r: f64,
	rot: f64,
	a_trans: Vect,
	a_rot: f64,
}

impl Ball {
	fn init_at_origin(r: f64) -> Ball {
		Ball {
			p: Point::new(0.0, 0.0),
			v: Vect::new(0.0, 0.0),
			r: r,
			rot: 0.0,
			a_trans: Vect::new(0.0, 0.0),
			a_rot: 0.0
		}
	}
}

// Assumes one 'wrap' will always be enough (the result is always within the bounds)
// Apparently generic numerical functions are pretty painful
fn bound_reflect<T: Add<Output=T> + Sub<Output=T> + PartialOrd + Copy>(x: T, min: T, max: T) -> T {
	if x > max {
		max - (x - max)
	} else if x < min {
		min + (min - x)
	} else {
		x
	}
}

fn wrap<T: Add<Output=T> + Sub<Output=T> + PartialOrd + Copy>(x: T, min: T, max: T) -> T {
	let mut y = x;
	while y > max {
		y = y - (max - min);
	}
	while y < min {
		y = y + max - min;
	}
	y
}

fn bound<T: PartialOrd + Copy>(x: T, min: T, max: T) -> T {
	if x < min {
		min
	} else if x > max {
		max
	} else {
		x
	}
}

impl Ball {
	fn update(&self, dt: f64) -> Ball {
		// TODO: global seeding
		let mut rng = rand::thread_rng();
		let range = rdist::Range::new(-0.01, 0.01);
		Ball {
			p: self.p + self.v*dt,
			v: self.v + self.a_trans*dt,
			r: self.r,
			rot: wrap(self.rot + self.a_rot*dt, -1.0, 1.0),
			a_trans: self.a_trans + Vect::new(range.ind_sample(&mut rng), range.ind_sample(&mut rng)),
			a_rot: self.a_rot + range.ind_sample(&mut rng)
		}
	}
}

struct App {
	balls: [Ball; 2]
}

impl App {
	fn init() -> App {
		App {
			balls: [Ball::init_at_origin(0.03), Ball::init_at_origin(0.03)]
		}
	}
}

fn main() {
	// TODO: width/height consts
	let mut window : PistonWindow = WindowSettings::new("Biobox", [400,400])
		.opengl(OpenGL::V2_1)
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
				let x = (b.p.x/2.0+0.5)*400.0;
				let y = (b.p.y/2.0+0.5)*400.0;
				Ellipse::new([1.0, 0.0, 0.0, 0.5])
					.draw(ellipse::circle(x, y, b.r*400.0), &c.draw_state, c.transform, g);
			}
			app.balls[0] = app.balls[0].update(1.0/60.0);
			app.balls[1] = app.balls[1].update(1.0/60.0);
		});
	}
}
