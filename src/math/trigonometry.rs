use std::f64::consts::PI;

pub const fn sin(degree: f64, terms: u32) -> f64 {
	let (mut result, mut sign, mut denominator, radians) = (0.0, 1.0, 1.0, degree * PI / 180.0);
	let mut numerator = radians;

	let mut current_term = 0;
	while current_term < terms {
		result += sign * (numerator / denominator);
		numerator *= radians * radians;
		denominator *= (2 * current_term + 2) as f64 * (2 * current_term + 3) as f64;
		sign = -sign;
		current_term += 1;
	}

	return result;
}

pub const fn cos(degree: f64, terms: u32) -> f64 {
	let (mut result, mut sign, mut denominator, mut numerator, radians) =
		(0.0, 1.0, 1.0, 1.0, degree * PI / 180.0);

	let mut current_term = 0;
	while current_term < terms {
		result += sign * (numerator / denominator);
		numerator *= radians * radians;
		denominator *= (2 * current_term + 1) as f64 * (2 * current_term + 2) as f64;
		sign = -sign;
		current_term += 1;
	}

	result
}

pub const fn tan(degree: f64, terms: u32) -> f64 {
	let angle_mod = degree % 180.0;

	if angle_mod == 90.0 || angle_mod == -90.0 {
		return f64::INFINITY;
	}

	sin(degree, terms) / cos(degree, terms)
}

pub const fn cot(degree: f64, terms: u32) -> f64 {
	if degree % 180.0 == 0.0 {
		return f64::INFINITY;
	}

	cos(degree, terms) / sin(degree, terms)
}
