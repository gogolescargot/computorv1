pub fn sqrt_newton(x: f64) -> f64 {
	if x <= 0.0 {
		return 0.0;
	}

	let mut y = x;
	for _ in 0..8 {
		y = 0.5 * (y + x / y);
	}
	return y;
}

pub fn absolute(x: f64) -> f64 {
	if x < 0. {
		return -x;
	}
	return x;
}
