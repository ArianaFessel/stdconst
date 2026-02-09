#![allow(incomplete_features)]
#![feature(generic_const_exprs, const_trait_impl, str_from_raw_parts)]

pub mod file;
pub mod math;
pub mod string;
pub mod vector;

pub use string::String;
pub use uninit_values_handling::*;
pub use vector::Vec;

/// needed by constant vector implementation due to unsupported Copy trait in original `MaybeUninit` structure
mod uninit_values_handling {
	use std::mem::MaybeUninit;

	#[derive(Debug)]
	pub struct Uninit<T>(pub MaybeUninit<T>);

	impl<T: Copy> Uninit<T> {
		pub const fn new(value: T) -> Self {
			Uninit(MaybeUninit::new(value))
		}

		pub const fn assume_init(&self) -> T {
			unsafe { self.0.assume_init() }
		}

		pub const fn as_mut_ptr(&mut self) -> *mut T {
			self.0.as_mut_ptr()
		}

		pub const fn uninit() -> Self {
			Self {
				0: MaybeUninit::<T>::uninit(),
			}
		}
	}

	impl<T: Copy> Clone for Uninit<T> {
		fn clone(&self) -> Self {
			Uninit(self.0)
		}
	}

	impl<T: Copy> Copy for Uninit<T> {}

	impl<T: std::fmt::Debug + Copy> std::fmt::Display for Uninit<T> {
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			write!(f, "{:?}", unsafe { self.0.assume_init() })
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn maths() {
		use crate::math::*;
		let terms = 10;

		for value in [
			("sin", 18.0, sin(18.0, terms)),
			("cos", 18.0, cos(18.0, terms)),
			("tan", 270.0, tan(270.0, terms)),
			("cot", 32.0, cot(32.0, terms)),
		] {
			dbg!("{} of {}° is {}", value.0, value.1, value.2);
		}
	}

	#[test]
	fn vector() {
		use crate::vector::*;

		let mut vector = Vec::<u8>::new(); // or `vec![]` with same usage as with standard `vec![]` macro. But of course, you will need also to specify the type of vector.
		vector.push(0);
		vector.push(1);

		println!("{:?}", vector.as_slice())
	}

	#[test]
	fn string() {
		use crate::string;
		use crate::string::*;

		println!(
			"{:?}",
			string!("   hello world	").trim().replace("h", "a").as_str()
		);

		println!(
			"{:?}; {:?}",
			string!("text, pattern").contains("pattern"),
			string!("text").contains("pattern")
		)
	}

	#[test]
	fn calc() {
		println!("{}", const { 210_326f32 / 606f32 });
		println!("{}", const { 67.0 * 9.81 * 320.0 });
		println!("{}", const { (67.0 * 9.81 * 320.0) / (10.1 * 60.0) });
	}
}
