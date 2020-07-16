mod utils;

use crate::utils::two_sum;

fn main() {
    println!("Hello, world!");
    let nums = vec![2, 7, 11, 13];
    let target = 9;

    let brute_force_result = two_sum::brute_force_two_sum(&nums, target);
    println!("The brute_force_result is {:?}", brute_force_result);

    let hash_map_result = two_sum::hash_map_two_sum(&nums, target);
    println!("The hash_map_result is {:?}", hash_map_result);

    let optimized_hash_map_result = two_sum::optimized_hash_map_two_sum(&nums, target);
    println!(
        "The optimized_hash_map_result is {:?}",
        optimized_hash_map_result
    );
}
