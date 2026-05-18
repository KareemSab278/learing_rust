mod challenges;

fn main() {
    let max_value = challenges::find_max(vec![3, 1, 4, 1, 5, 9]);
    println!("Max value: {:?}", max_value);
}