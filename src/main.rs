// https://leetcode.ca/all/problems.html
// fn find_two_sum(mut input: Vec<i32>, target: i32) -> Result<Vec<i32>, [i32; 2]> {
//     input.sort();
//     let mut output = Vec::new();
//     let mut start: usize = 0 as usize;
//     let mut end: usize = input.len() - 1;

//     while start < end {
//         if input[end] + input[start] > target {
//             end -= 1;
//         } else if input[end] + input[start] < target {
//             start += 1;
//         } else if input[end] + input[start] == target {
//             output.push(start as i32);
//             output.push(end as i32);
//             return Ok(output);
//         }
//     }
//     Err([0, 0])
// }
// fn main() {
//     // two sum
//     let input = vec![1, 2, 3, 4, 5, 6];
//     let target = 9;
//     match find_two_sum(input, target) {
//         Ok(result) => println!("Found two numbers: {:?}", result),
//         Err(_) => println!("No two numbers found that sum to the target."),
//     }
// }

fn rev_int(input: i32) -> i32 {
    let mut output: Vec<u32> = input
        .to_string()
        .chars()
        .map(|chr| chr.to_digit(10).unwrap())
        .collect();
    output.reverse();
    let reversed_str: String = output
        .iter()
        .map(|d| d.to_string())
        .collect();
    let reversed_int: i32 = reversed_str.parse().unwrap();
    println!("{}", reversed_int);
    reversed_int
}

fn main() {
    rev_int(32);
}
