extern crate piston;
extern crate graphics;
extern crate piston_window;

use piston_window::*;

fn main() {
	println!("Hello, world!");

	let mut window : PistonWindow = WindowSettings::new("Biobox", [400,300])
		.opengl(OpenGL::V3_2)
		.exit_on_esc(true)
		.vsync(true)
		.build()
		.unwrap(); // TODO: error handling

	window.set_ups(60);

	while let Some(e) = window.next() {
	}
}
