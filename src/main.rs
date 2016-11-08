#[macro_use] extern crate conrod;
extern crate piston_window;

use piston_window::{EventLoop, OpenGL, PistonWindow, UpdateEvent, WindowSettings};

fn main() {
	println!("Hello, world!");

	let opengl = OpenGL::V3_2;
	let mut window: PistonWindow = WindowSettings::new("Biobox", [400,300])
		.opengl(opengl)
		.exit_on_esc(true)
		.vsync(true)
		.build()
		.unwrap();

	window.set_ups(60);

	while let Some(e) = window.next() {
	}
}
