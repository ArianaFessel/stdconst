use std::mem::MaybeUninit;

/// reimplementation of `std::alloc::Vec`.
/// also has `vec![]` macro with same usage as `vec![]` from standard library has.
///
/// # Underhood
/// it has two generic parameters: first is type and second
/// is maximal size of vector, which is set by default to 128.
///
/// the last index of initialized value is saved in "length" field.
/// Changing it can cause undefined behavior and errors.
///
/// # examples
/// ```no_run
/// let vector: Vec<u8> = vec![1, 0, 80];
///
/// println!("{:?}", vector.as_slice()); // [1, 0, 80]
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Vec<T: Copy, const N: usize = 128> {
	pub data: [MaybeUninit<T>; N],
	pub length: usize,
}

impl<T: Copy, const N: usize> Vec<T, N> {
	/// creates new instance of vector with default size: 128
	pub const fn new() -> Self {
		Self {
			data: [MaybeUninit::uninit(); N],
			length: 0,
		}
	}

	/// Creates a new vector with the specified capacity and copies
	/// the contents of the current vector into it.
	///
	/// # Panics
	///
	/// New size should be bigger than current.
	pub const fn resize<const N2: usize>(&self) -> Vec<T, N2> {
		if N2 < N {
			panic!("new size is smaller than previous")
		}

		let slice = self.as_slice();
		let mut new_vector = Vec::<T, N2>::new();
		let mut index = 0;

		while index < self.length && index < N2 {
			new_vector.set(index, slice[index]);
			index += 1;
		}

		new_vector.length = if self.length < N2 { self.length } else { N2 };
		new_vector
	}

	/// creates new instance of vector from specified slice
	pub const fn from_slice(value: &[T]) -> Self {
		let mut vector = Self::new();
		let length = value.len();
		let mut index = 0;

		while index < length && vector.length < N {
			vector.push(value[index]);
			index += 1;
		}

		vector
	}

	pub const fn as_slice(&self) -> &[T] {
		unsafe { core::slice::from_raw_parts(self.data.as_ptr() as *const T, self.length) }
	}

	pub const fn slice(&self, start_index: usize, end_index: usize) -> [MaybeUninit<T>; N] {
		assert!(end_index <= self.len());
		assert!(start_index <= self.len());
		assert!(start_index <= end_index);

		let mut data = Vec::<T, N>::new();
		let length = start_index + end_index;

		let mut index = start_index;
		while index < length {
			data.push(unsafe { self.data[index].assume_init() });
			index += 1;
		}

		data.data
	}

	pub const fn push(&mut self, value: T) {
		if self.length >= N {
			panic!("vector is too small");
		}

		self.data[self.length].write(value);
		self.length += 1;
	}

	pub const fn get(&self, index: usize) -> Option<T>
	where
		T: Copy,
	{
		if index < self.length {
			Some(unsafe { self.data[index].assume_init() })
		} else {
			None
		}
	}

	pub const fn set(&mut self, index: usize, new_value: T) {
		if index < self.length {
			self.data[index] = MaybeUninit::new(new_value);
		}
	}

	/// returns index of last initialized type in slice.
	pub const fn len(&self) -> usize {
		return self.length;
	}

	/// returns size of vector.
	pub const fn size(&self) -> usize {
		return N;
	}
}

#[macro_export]
macro_rules! vec {
	[ $( $value:expr ),* ] => {{
		Vec::from_slice(&[$($value,)*])
	}};
}

#[test]
fn test() {
	println!(
		"{:#?}",
		const {
			let mut vector: Vec<u8, 1000000> = vec![18, 12, 0, 9, 3];

			let mut index = 0;
			while index < vector.len() {
				let current = vector.get(index).unwrap();
				vector.set(index, current + 1);
				index += 1;
			}

			vector
		}
		.as_slice()
	);
}
