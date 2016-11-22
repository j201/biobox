use std::ops::*;

// Assumes one 'wrap' will always be enough (the result is always within the bounds)
// Apparently generic numerical functions are pretty painful
pub fn bound_reflect<T: Add<Output=T> + Sub<Output=T> + PartialOrd + Copy>(x: T, min: T, max: T) -> T {
	if x > max {
		max - (x - max)
	} else if x < min {
		min + (min - x)
	} else {
		x
	}
}

pub fn wrap<T: Add<Output=T> + Sub<Output=T> + PartialOrd + Copy>(x: T, min: T, max: T) -> T {
	let mut y = x;
	while y > max {
		y = y - (max - min);
	}
	while y < min {
		y = y + max - min;
	}
	y
}

pub fn bound<T: PartialOrd + Copy>(x: T, min: T, max: T) -> T {
	if x < min {
		min
	} else if x > max {
		max
	} else {
		x
	}
}
