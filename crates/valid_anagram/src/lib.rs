use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
	if s.len() != t.len() {
		return false;
	}

	let mut map: HashMap<char, i64> = HashMap::new();

	for (a, b) in s.chars().zip(t.chars()) {
		*map.entry(a).or_default() += 1;
		*map.entry(b).or_default() -= 1;
	}

	map.into_values().all(|count| count == 0)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn valid() {
		assert!(is_anagram(String::from("anagram"), String::from("nagaram")));
	}

	#[test]
	fn invalid() {
		assert!(!is_anagram(String::from("rat"), String::from("car")));
	}

	#[test]
	fn different_length() {
		assert!(!is_anagram(String::from("Hello"), String::from("helllllo")))
	}
}
