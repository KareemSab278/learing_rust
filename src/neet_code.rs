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


//return the index of the numbers that add up to target
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>{
    // sort the vec first, then find out where the two numbers add up
    // then find the idnexes for those nums in the original vec

    let mut first = 0;
    let mut second = nums.len() as i32;
    let mut output: Vec<i32> = Vec::new();
    second -= 1;
    
    while first < second {
        if nums[first as usize] + nums[second as usize] > target{
            second -= 1
        }
        if nums[first as usize] + nums[second as usize] < target{
            first += 1;
        }
        else {

    output.push(first);
    output.push(second);
    return output;

        }
    }

    output
}


pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut groups: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();

    for s in strs {
        // create sorted version of the word
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_unstable();

        let key: String = chars.into_iter().collect();

        groups.entry(key).or_default().push(s);
    }

    groups.into_values().collect()
}