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

// fn rev_int(input: i32) -> i32 {
//     let mut output: Vec<u32> = input
//         .to_string()
//         .chars()
//         .map(|chr| chr.to_digit(10).expect("Invalid number inserted"))
//         .collect();

//     output.reverse();

//     let output_collected_str: String = output
//         .iter()
//         .map(|o| o.to_string())
//         .collect();

//     let output_collected_int: i32 = output_collected_str
//         .parse()
//         .expect("Could not convert the output str to int");
//     println!("{}", output_collected_int);
//     return output_collected_int;
// }

// fn main() {
//     rev_int(32);
// }

// fn longest_substr(input: String) -> i16 {
//     let mut stack: Vec<char> = Vec::new();
//     let mut iter: usize = 0;
//     let input_c: Vec<char> = input.chars().collect();
//     while iter < input.len() {
//         if !stack.contains(&input_c[iter]) {
//             stack.push(input_c[iter]);
//         }
//         iter += 1;
//     }
//     let stacklen= stack.len();
//     let output: i16 = stacklen as i16;
//     println!("{}", output);
//     return output;

//     // i just realized i coul have used a set... but idk how in rust yet lol
// }

// fn main() {
//     let test_str: String = "veveveveveveve".to_string();
//     longest_substr(test_str);
// }

fn find_medean(input1: Vec<i32>, input2: Vec<i32>)-> Result<f32, bool>{
    // rust has a spread operator: y.iter().chain(&input);
    // OR: y.iter().chain(&x).map(|&x|x).collect();
    // chain is an iterator that combines two iterators together like a chain. <- !!!
    if input1.len() == 0 || input2.len() == 0 {
        return Err(false);
    }
    let combined_input: Vec<i32> = input1.iter().chain(&input2).map(|&x|x).collect(); // this works apparently so thats cool.
    let input_len = combined_input.len() as f32;
    let mut output: f32 = 0.0;
    
    for num in combined_input {
        output += num as f32;
    }

    output = output/input_len;

    // Add up all of the numbers and divide by the number of numbers in the data set.

    println!("output: {}", output);
    Ok(output)
}

fn main(){
    let input1: Vec<i32> = [1,2,3,4,5,].to_vec();
    let input2: Vec<i32> = [6,7,8, 9, 10, 11, 12, 13, 90].to_vec();
    let medean = find_medean(input1, input2);
    if medean == Err(false){
        println!("one of your inputs was empty bruh");
    }
}



// ====================================================================================================================================
// NOTES: 
// combine vecs: let combined_input: Vec<i32> = input1.iter().chain(&input2).map(|&x|x).collect();