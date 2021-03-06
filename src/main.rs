extern crate piston_window;
extern crate nalgebra as na;
extern crate rand;

#[allow(dead_code)]
mod utils;
#[allow(dead_code)]
mod ann_graph;

use piston_window::*;
use rand::distributions::IndependentSample;
use rand::distributions as rdist;
use std::f64::consts::PI;
use ann_graph::AnnGraph;
use na::Norm;

type Vect = na::Vector2<f64>;
type Point = na::Point2<f64>;

const LINK_REST_LEN: f64 = 0.02;
const LINK_K: f64 = 0.5;
const FOOD_PERIOD: f64 = 1.0;
const FOOD_RADIUS: f64 = 3.0;

#[derive(Clone)]
struct Link {
	activation: f64,
}

struct Cell {
	p: Point,
	v: Vect,
	r: f64,
	rho: f64,
	f: Vect,
}

impl Cell {
	fn init_at(p: Point, r: f64) -> Cell {
		Cell {
			p: p,
			v: Vect::new(0.0, 0.0),
			r: r,
			rho: 10.0, // TODO: parameterize?
			f: Vect::new(0.0, 0.0),
		}
	}
	fn update(&self, neighbours: Vec<&Cell>, dt: f64) -> Cell {
		// TODO: global seeding
		let mut rng = rand::thread_rng();
		let range = rdist::Range::new(-0.01, 0.01);
		let f_link: Vect = neighbours.iter().map(|c| {
			let link_vec = c.p - self.p;
			let link_len = link_vec.norm();
			if link_len < LINK_REST_LEN {
				Vect::new(0.0, 0.0)
			} else {
				(link_len - LINK_REST_LEN) / link_len * LINK_K * link_vec
			}
		}).fold(Vect::new(0.0, 0.0), |acc, el| { acc + el }); // No Sum impl :(
		let f_total = f_link + self.f;
		let m = PI * self.r * self.r * self.rho;
		Cell {
			p: self.p + self.v*dt,
			v: self.v + f_total/m*dt,
			f: self.f + Vect::new(range.ind_sample(&mut rng), range.ind_sample(&mut rng)),
			.. *self
		}
	}
}

type Critter = AnnGraph<Cell, Link>;

impl Critter {
	fn update(&self, dt: f64) -> Critter {
		self.modify_nodes(|id, n| {
			n.update(self.neighbours(id), dt)
		})
	}
}

struct Food {
	p: Point,
	val: f64
}

impl Food {
	fn new_rand() -> Food {
		let mut rng = rand::thread_rng();
		let range = rdist::Range::new(-1.0, 1.0);
		Food {
			p: Point::new(range.ind_sample(&mut rng), range.ind_sample(&mut rng)),
			val: 1.0
		}
	}
}

// TODO: collision
// - extract critter updating into an App-level update function
// - after movement, check for overlapping cells
//   - move these cells apart and reflect the components of their velocities parallel to the line between their centres (inelastic)
// - if a critter collides with food, eat it

struct App {
	t: f64,
	critters: Vec<Critter>,
	food: Vec<Food>,
	t_next_food: f64
}

impl App {
	fn init() -> App {
		let mut critters = vec![];
		let n_critters = 2;

		for i in 0..n_critters {
			let cell1 = Cell::init_at(Point::new(-0.1, -0.5 + i as f64), 0.03);
			let cell2 = Cell::init_at(Point::new(0.1, -0.5 + i as f64), 0.03);
			let mut critter = AnnGraph::new();
			let id1 = critter.add_node(cell1, vec![]);
			critter.add_node(cell2, vec![(id1, Link { activation: 1.0 })]);
			critters.push(critter);
		}
		App {
			critters: critters,
			food: vec![],
			t: 0.0,
			t_next_food: 0.0
		}
	}
	fn update(&mut self, dt: f64) {
		self.t += dt;
		self.critters = self.critters.iter().map(|c| c.update(dt)).collect();
		if self.t >= self.t_next_food {
			self.food.push(Food::new_rand());
			self.t_next_food += FOOD_PERIOD;
		}
	}
}

// Modifies point from a (-1,1) range to a (0,w/h) range
fn on_screen(p: &Point, width: f64, height: f64) -> Point {
	Point::new((p.x + 1.0) * width / 2.0, (p.y + 1.0) * height / 2.0)
}

fn main() {
	let width = 400.0;
	let height = 400.0;

	let mut window : PistonWindow = WindowSettings::new("Biobox", [width as u32, height as u32])
		.opengl(OpenGL::V2_1)
		.exit_on_esc(true)
		.vsync(true)
		.build()
		.unwrap(); // TODO: error handling

	window.set_ups(60);

	let mut app = App::init();

	while let Some(e) = window.next() {
		match e {
			Event::Update(UpdateArgs{ dt }) => {
				app.update(dt);
			}
			_ => {}
		}
		window.draw_2d(&e, |c, g| {
			clear(color::WHITE, g);
			for f in &app.food {
				let p = on_screen(&f.p, width, height);
				Ellipse::new([0.5, 1.0, 0.5, 1.0])
					.draw(ellipse::circle(p.x, p.y, FOOD_RADIUS), &c.draw_state, c.transform, g);
			}
			for cr in &app.critters {
				for e in cr.edges() {
					let (n1, n2) = cr.ends(e);
					let p1 = on_screen(&n1.p, width, height);
					let p2 = on_screen(&n2.p, width, height);
					Line::new([0.0, 0.0, 0.0, 0.5], 1.0)
						.draw([p1.x, p1.y, p2.x, p2.y], &c.draw_state, c.transform, g);
				}
				for n in cr.nodes() {
					let p = on_screen(&n.p, width, height);
					Ellipse::new([1.0, 0.5, 0.5, 1.0])
						.draw(ellipse::circle(p.x, p.y, n.r*width), &c.draw_state, c.transform, g);
				}
			}
		});
	}
}
