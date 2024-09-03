pub fn is_palindrome(s: String) -> bool {
	if s.trim().is_empty() {
		return true;
	}

	let bytes = s.as_bytes();

	let mut left_pointer = 0;
	let mut right_pointer = bytes.len() - 1;

	while left_pointer < right_pointer {
		if !bytes[left_pointer].is_ascii_alphanumeric() {
			left_pointer += 1;
			continue;
		}

		if !bytes[right_pointer].is_ascii_alphanumeric() {
			right_pointer -= 1;
			continue;
		}

		if bytes[left_pointer].to_ascii_lowercase() != bytes[right_pointer].to_ascii_lowercase() {
			return false;
		}

		left_pointer += 1;
		right_pointer -= 1;
	}

	true
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn case_1() {
		assert!(is_palindrome(String::from("A man, a plan, a canal: Panama")));
	}

	#[test]
	fn case_2() {
		assert!(!is_palindrome(String::from("race a car")));
	}

	#[test]
	fn case_3() {
		assert!(is_palindrome(String::from(" ")));
	}

	#[test]
	fn case_4() {
		assert!(is_palindrome(String::from("f f")));
	}
}
