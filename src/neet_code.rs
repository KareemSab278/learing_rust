// gonna try doing 1 neetcode problem a day.

pub fn has_duplicate(nums: Vec<i32>) -> bool { // return true if there are duplicates in the array, false otherwise
    // do a set from the array and compare the length of the set to the length of the array
    let set: std::collections::HashSet<i32> = nums.clone().into_iter().collect();
    set.len() != nums.len()
}

pub fn valid_anagram(phrase: &str, phrase1: &str) -> bool {
	// has same letters
	let len_check = phrase.len() == phrase1.len();
		if len_check == false {
			return false
}

// build a hashmap from the words now
	let mut hashmap = std::collections::HashMap::new();
	let mut hashmap1 = std::collections::HashMap::new();

	for c in phrase.clone().chars() {
		*hashmap.entry(c).or_insert(0) += 1;
	}

	for c in phrase1.clone().chars() {
		*hashmap1.entry(c).or_insert(0) += 1;
	}

	hashmap == hashmap1

}
