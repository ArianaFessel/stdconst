use super::trigonometry::{cos, sin};
pub type Point2D = (f64, f64);
pub type Point3D = (f64, f64, f64);

/// rotates first point around second point
pub const fn rotate2d(points: [Point2D; 2], angle: f64) -> Point2D {
	let ((vx, vy), (px, py)) = (points[0], points[1]);
	let (dx, dy) = (vx - px, vy - py);
	let rotated_x = dx * cos(angle, 10) - dy * sin(angle, 10);
	let rotated_y = dx * sin(angle, 10) + dy * cos(angle, 10);

	return (rotated_x + px, rotated_y + py);
}

/// rotates first point around second point and simply ignores Z coordinate
pub const fn rotate3d(points: [Point3D; 2], angle: f64) -> Point3D {
	let ((vx, vy, vz), (px, py, pz)) = (points[0], points[1]);
	let (dx, dy, dz) = (vx - px, vy - py, vz - pz);

	let rotated_x = dx * cos(angle, 10) - dy * sin(angle, 10);
	let rotated_y = dx * sin(angle, 10) + dy * cos(angle, 10);
	let rotated_z = dz;

	return (rotated_x + px, rotated_y + py, rotated_z + pz);
}
