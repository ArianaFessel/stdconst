#![allow(unused)]
use crate::vector::*;

/// reimplementation of std::string::String
/// for work in constant context.
///
/// # limitation
/// string length should be always
/// less than 128 chars (same as
/// default max size of vector),
/// but it's possible to change it
/// via generic parameter.
#[derive(Debug, Clone, Copy)]
pub struct String<const N: usize = 128> {
	data: Vec<u8, N>,
}

impl<const N: usize> String<N> {
	///
	/// Constructor Methods
	///

	pub const fn new() -> Self {
		String {
			data: Vec::<u8, N>::new(),
		}
	}

	pub const fn from(value: &str) -> String<N> {
		let mut data = Vec::<u8, N>::new();
		let bytes = value.as_bytes();

		let mut index = 0;
		while index < value.len() {
			data.push(bytes[index] as u8);
			index += 1;
		}

		String { data }
	}

	/// Constructs a new `String` from a slice of bytes.
	///
	/// Behavior is undefined if using invalid bytes due to the usage of unsafe `std::str::from_raw_parts`.
	pub const fn from_bytes(bytes: &[u8]) -> Self {
		String::from(unsafe { std::str::from_raw_parts(bytes.as_ptr(), bytes.len()) })
	}

	/// Constructs a new `String` from UTF-8 bytes.
	///
	/// Behavior is undefined if using invalid bytes due to the usage of unsafe `std::str::from_utf8_unchecked`.
	pub const fn from_utf8(vec: Vec<u8>) -> Result<String, &'static str> {
		match std::str::from_utf8(&vec.as_slice()) {
			Ok(..) => Ok(String::from(unsafe {
				std::str::from_utf8_unchecked(&vec.as_slice())
			})),
			Err(..) => Err("error occured while creating string from utf8"),
		}
	}

	///
	/// Convertation Between Different Types
	///
	pub const fn as_bytes(&self) -> &[u8] {
		self.data.as_slice()
	}

	/// converts this structure into "&str".
	///
	/// # safety
	///
	/// Behavior is undefined if this string contains invalid characters due to usage of unsafe `core::str::from_utf8_unchecked`.
	pub fn as_str(&self) -> Result<&str, core::str::Utf8Error> {
		core::str::from_utf8(self.as_bytes())
	}

	///
	/// Changing String
	///
	pub const fn push_char(&mut self, c: char) {
		let mut buf = [0u8; 4];
		let encoded = c.encode_utf8(&mut buf);

		let bytes = encoded.as_bytes();

		let mut i = 0;
		while i < bytes.len() {
			self.data.push(bytes[i]);
			i += 1;
		}
	}

	pub const fn push_str(&mut self, value: &str) {
		let mut index = 0;
		while index < value.len() {
			self.push_char(value.as_bytes()[index] as char);
			index += 1;
		}
	}

	pub const fn trim(&self) -> Self {
		let mut start = 0;
		let length = self.len();
		let mut end = length;

		while start < length && self.is_whitespace(start) {
			start += 1;
		}

		while end > start && self.is_whitespace(end - 1) {
			end -= 1;
		}

		let mut string = String::new();

		let mut index = start;
		let mut string_index = 0;
		while index < end {
			let character = self.data.get(index).unwrap() as char;
			string.push_char(character);
			string_index += 1;
			index += 1;
		}

		string
	}

	///
	/// Getters
	///
	pub const fn get(&self, index: usize) -> Option<u8> {
		self.data.get(index)
	}

	pub const fn len(&self) -> usize {
		return self.data.len();
	}

	///
	/// Predicate Functions
	///
	pub const fn starts_with(&self, value: char) -> bool {
		return self.as_bytes()[0] == value as u8;
	}

	pub const fn ends_with(&self, value: char) -> bool {
		return self.as_bytes()[self.len() - 1] == value as u8;
	}

	pub const fn is_whitespace(&self, index: usize) -> bool {
		match self.get(index) {
			Some(value) => return value.is_ascii_whitespace(),
			None => panic!("invalid index"),
		}
	}

	pub const fn is_empty(&self) -> bool {
		self.data.len() == 0
	}

	pub const fn eq(&self, other: &String) -> bool {
		if self.len() != other.len() {
			return false;
		}

		let mut index: usize = 0;
		while index < self.len() {
			let this = self.get(index).unwrap();
			let other = other.get(index).unwrap();

			if this == other {
				return false;
			}

			index += 1;
		}

		return true;
	}

	pub const fn contains(&self, pattern: &str) -> bool {
		let pattern: String = String::from(pattern);

		if pattern.len() == 0 || self.eq(&pattern) {
			return true;
		} else if self.len() < pattern.len() {
			return false;
		}

		let mut index = 0;
		while index <= self.len() - pattern.len() {
			let end_index: usize = index + pattern.len();
			let slice: String<N> = self.slice(index, end_index + 1);

			if slice.eq(&pattern) {
				return true;
			}

			index += 1;
		}

		return false;
	}

	///
	/// Iterator Methods
	///
	pub const fn find(&self, pattern: &str) -> Option<usize> {
		if pattern.len() == 0 || pattern.len() > self.len() {
			return None;
		}

		let pattern_bytes = pattern.as_bytes();

		let mut index = 0;
		while index <= self.len() - pattern.len() {
			let mut match_found = true;

			let mut inner_index = 0;
			while inner_index < pattern.len() {
				let src_char = self.get(index + inner_index).unwrap();
				let pattern_char = pattern_bytes[inner_index];

				if src_char != pattern_char {
					match_found = false;

					break;
				}

				inner_index += 1;
			}

			if match_found {
				return Some(index);
			}

			index += 1;
		}

		None
	}

	pub const fn find_char(&self, pattern: char) -> Option<usize> {
		let pattern = pattern as u8;
		let bytes = self.as_bytes();

		let mut index: usize = 0;
		while index < self.len() {
			if bytes[index] == pattern {
				return Some(index);
			}

			index += 1;
		}

		return None;
	}

	pub const fn split(&self, pattern: &'static str) -> Vec<String<N>> {
		let mut result: Vec<String<N>> = Vec::new();
		let pattern = String::<N>::from(pattern);

		if pattern.len() == 0 {
			return result;
		}

		let (mut start, mut index) = (0, 0);

		while index <= self.len() - pattern.len() {
			let mut match_found = true;

			let mut inner_index = 0;
			while inner_index < pattern.len() {
				if self.get(index + inner_index).unwrap() != pattern.get(inner_index).unwrap() {
					match_found = false;
					break;
				}
				inner_index += 1;
			}

			if match_found {
				// "+ 1" in "index + 1" is needed to include last character.
				let substring: String<N> = self.slice(start, index + 1);
				result.push(substring);
				start = index + pattern.len();
				index = start;
			} else {
				index += 1;
			}
		}

		if start < self.len() {
			// "+ 1" here is needed due to the same reason
			result.push(self.slice(start, self.len() + 1));
		}

		result
	}

	pub const fn split_char(&self, pattern: char) -> Vec<String<N>> {
		let mut result: Vec<String<N>> = Vec::new();
		let mut index: usize = 0;
		let mut previous_char_index: usize = 0;
		let pattern = pattern as u8;

		while index < self.len() {
			let current_char: u8 = self.data.get(index).unwrap();
			let is_last_index: bool = index == self.len() - 1;

			let increment: u8 = {
				let is_first_match: bool = current_char == pattern && previous_char_index == 0;

				match is_first_match {
					true => 0,
					false => 1,
				}
			};

			if current_char == pattern || is_last_index {
				if is_last_index {
					index += 1;
				}

				let slice: String<N> = self.slice(previous_char_index + increment as usize, index + 1);
				result.push(slice);
				previous_char_index = index;
			}

			index += 1;
		}

		return result;
	}

	pub const fn slice(&self, start: usize, end: usize) -> Self {
		let value: &[u8] = self.as_bytes();
		let mut result: String<N> = String::new();
		let mut index: usize;

		index = 0;
		while index < end - start - 1 {
			let character = value[start + index] as char;
			result.push_char(character);
			index += 1;
		}

		result
	}

	pub const fn replace(&self, from: &str, to: &str) -> Self {
		let mut result = String::new();
		let length = self.data.len();
		let (old_bytes, new_bytes) = (from.as_bytes(), to.as_bytes());
		let (old_len, new_len) = (old_bytes.len(), new_bytes.len());
		let mut index = 0;

		while index < length {
			let (mut match_found, mut inner_index) = (true, 0);

			if index + old_len > length {
				index += 1;

				continue;
			}

			while inner_index < old_len {
				if self.get(index + inner_index).unwrap() != old_bytes[inner_index] {
					match_found = false;
					break;
				}
				inner_index += 1;
			}

			if match_found {
				let mut inner_index = 0;
				while inner_index < new_len {
					result.push_char(new_bytes[inner_index] as char);
					inner_index += 1;
				}
				index += old_len;
				continue;
			}

			result.push_char(self.get(index).unwrap() as char);
			index += 1;
		}

		result
	}
}

#[macro_export]
macro_rules! string {
	($literal:expr) => {{
		String::<{ $literal.len() }>::from($literal)
	}};
}

#[test]
fn split() {
	const STRING: String = String::from("hello world! so so");
	const SPLITTED_VALUE: Vec<String> = STRING.split(" ");

	for string in SPLITTED_VALUE.as_slice() {
		println!("{}", string.as_str().unwrap());
	}
}

#[test]
fn find() {
	const STRING: String = String::from("hello world! so so");

	println!("{}", STRING.find("o s").unwrap())
}
