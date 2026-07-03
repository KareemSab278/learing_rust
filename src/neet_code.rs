// gonna try doing 1 neetcode problem a day.
use std::collections::{ HashMap, HashSet };

#[allow(dead_code)]
pub fn has_duplicate(nums: Vec<i32>) -> bool {
    // return true if there are duplicates in the array, false otherwise
    // do a set from the array and compare the length of the set to the length of the array
    let set: HashSet<i32> = nums.clone().into_iter().collect();
    set.len() != nums.len()
}

#[allow(dead_code)]
pub fn valid_anagram(phrase: &str, phrase1: &str) -> bool {
    // has same letters
    let len_check = phrase.len() == phrase1.len();
    if len_check == false {
        return false;
    }

    // build a hashmap from the words now
    let mut hashmap = HashMap::new();
    let mut hashmap1 = HashMap::new();

    for c in phrase.chars() {
        *hashmap.entry(c).or_insert(0) += 1;
    }

    for c in phrase1.chars() {
        *hashmap1.entry(c).or_insert(0) += 1;
    }

    hashmap == hashmap1
}

//return the index of the numbers that add up to target
#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // sort the vec first, then find out where the two numbers add up
    // then find the idnexes for those nums in the original vec

    let mut first = 0;
    let mut second = nums.len() as i32;
    let mut output: Vec<i32> = Vec::new();
    second -= 1;

    while first < second {
        if nums[first as usize] + nums[second as usize] > target {
            second -= 1;
        }
        if nums[first as usize] + nums[second as usize] < target {
            first += 1;
        } else {
            output.push(first as i32);
            output.push(second as i32);
            return output;
        }
    }

    output
}

#[allow(dead_code)]
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();

    for s in strs {
        // create sorted version of the word
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_unstable();

        let key: String = chars.into_iter().collect();

        groups.entry(key).or_default().push(s);
    }

    groups.into_values().collect()
}

#[allow(dead_code)]
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut counts = HashMap::new();

    for n in nums {
        *counts.entry(n).or_insert(0) += 1;
    }

    let mut freq: Vec<(i32, i32)> = counts.into_iter().collect();

    freq.sort_by(|a, b| b.1.cmp(&a.1));

    freq.into_iter()
        .take(k as usize)
        .map(|(num, _)| num)
        .collect()
}
#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn encode(s: Vec<String>) -> String {
        s.join("@^^^@")
    }

    pub fn decode(s: String) -> Vec<String> {
        s.split("@^^^@")
            .map(|s| s.to_string())
            .collect()
    }
}

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();

    let mut filter_pos: i32 = 0; // imma use this to filter the current position and add it to the output.

    for _ in 0..nums.len() {
        output.push(
            nums
                .iter()
                .filter(|&&x| x != nums[filter_pos as usize])
                .product()
        );
        filter_pos += 1;
    }

    output
}
