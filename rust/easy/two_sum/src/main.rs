use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let nums = vec![2, 7, 11, 13];
    let target = 9;

    let brute_force_result = brute_force_two_sum(nums, target);
    println!("The brute_force_result is {:?}", brute_force_result);

    let hash_map_result = hash_map_two_sum(vec![2, 7, 11, 13], target);
    println!("The hash_map_result is {:?}", hash_map_result);

    let optimized_hash_map_result = optimized_hash_map_two_sum(vec![2, 7, 11, 13], target);
    println!(
        "The optimized_hash_map_result is {:?}",
        optimized_hash_map_result
    );
}

fn brute_force_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, &a) in nums.iter().enumerate() {
        for (j, &b) in nums.iter().enumerate() {
            if i != j && a + b == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    return vec![];
}

fn hash_map_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hm = HashMap::new();
    for (i, &val) in nums.iter().enumerate() {
        hm.insert(val, i as i32);
    }

    for (i, &val) in nums.iter().enumerate() {
        let look = target - val;
        if let Some(&j) = hm.get(&look) {
            let pos = j as usize;
            if pos != i {
                return vec![i as i32, j];
            }
        }
    }

    vec![]
}

fn optimized_hash_map_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hm = HashMap::new();

    for (i, &val) in nums.iter().enumerate() {
        let look = target - val;
        if let Some(&j) = hm.get(&look) {
            let pos = j as usize;
            if pos != i {
                return vec![i as i32, j];
            }
        }
        hm.insert(val, i as i32);
    }

    vec![]
}
