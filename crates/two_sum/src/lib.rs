use std::collections::HashMap;

#[must_use]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
	let mut map = HashMap::new();

	for (i, number) in nums.into_iter().enumerate() {
		let diff = target - number;

		if let Some(&j) = map.get(&diff) {
			#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
			return vec![i as i32, j as i32];
		}

		map.insert(number, i);
	}

	unreachable!()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn case_1() {
		let result = two_sum(vec![2, 7, 11, 15], 9);
		assert_eq!(result, [0, 1]);
	}

	#[test]
	fn case_2() {
		let result = two_sum(vec![3, 2, 4], 6);
		assert_eq!(result, [1, 2]);
	}

	#[test]
	fn case_3() {
		let result = two_sum(vec![3, 3], 6);
		assert_eq!(result, [0, 1]);
	}
}
