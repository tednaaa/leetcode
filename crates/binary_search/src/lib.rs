use std::cmp::Ordering;

pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
	let mut left_pointer = 0;
	let mut right_pointer = nums.len();

	while left_pointer < right_pointer {
		let middle = left_pointer + (right_pointer - left_pointer) / 2;

		match target.cmp(&nums[middle]) {
			Ordering::Equal => return middle as i32,
			Ordering::Less => right_pointer = middle,
			Ordering::Greater => left_pointer = middle + 1,
		}
	}

	-1
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn case_target_in_middle() {
		let result = binary_search(vec![1, 3, 5, 7, 9], 5);
		assert_eq!(result, 2);
	}

	#[test]
	fn case_target_in_start() {
		let result = binary_search(vec![1, 3, 5, 7, 9, 11], 11);
		assert_eq!(result, 5);
	}

	#[test]
	fn case_target_in_end() {
		let result = binary_search(vec![1, 3, 5, 7, 9], 9);
		assert_eq!(result, 4);
	}

	#[test]
	fn case_not_found() {
		let result = binary_search(vec![1, 3, 5, 7, 9, 11], 0);
		assert_eq!(result, -1);
	}
}
