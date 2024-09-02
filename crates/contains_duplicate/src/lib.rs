use std::collections::HashSet;

#[must_use]
pub fn contains_duplicate(numbers: Vec<i32>) -> bool {
	let mut set = HashSet::new();

	for number in numbers {
		if set.contains(&number) {
			return true;
		}

		set.insert(number);
	}

	false
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn one_duplicate() {
		let contains = contains_duplicate(vec![1, 2, 3, 1]);
		assert!(contains);
	}

	#[test]
	fn many_duplicates() {
		let contains = contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]);
		assert!(contains);
	}

	#[test]
	fn no_duplicates() {
		let contains = contains_duplicate(vec![1, 2, 3, 4]);
		assert!(!contains);
	}

	#[test]
	fn empty_vec() {
		let contains = contains_duplicate(vec![]);
		assert!(!contains);
	}

	#[test]
	fn one_element() {
		let contains = contains_duplicate(vec![1]);
		assert!(!contains);
	}
}
