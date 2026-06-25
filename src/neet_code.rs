// gonna try doing 1 neetcode problem a day.

pub fn has_duplicate(nums: Vec<i32>) -> bool { // return true if there are duplicates in the array, false otherwise
    // do a set from the array and compare the length of the set to the length of the array
    let set: std::collections::HashSet<i32> = nums.clone().into_iter().collect();
    set.len() != nums.len()
}