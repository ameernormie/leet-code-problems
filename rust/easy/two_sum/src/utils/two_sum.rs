use std::collections::HashMap;

pub fn brute_force_two_sum(nums: &Vec<i32>, target: i32) -> Vec<i32> {
    for (i, &a) in nums.iter().enumerate() {
        for (j, &b) in nums.iter().enumerate() {
            if i != j && a + b == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    return vec![];
}

pub fn hash_map_two_sum(nums: &Vec<i32>, target: i32) -> Vec<i32> {
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

pub fn optimized_hash_map_two_sum(nums: &Vec<i32>, target: i32) -> Vec<i32> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_sum_using_brute_force_for_valid_list() {
        let nums = vec![2, 7, 13, 78];
        let target = 9;
        assert_eq!(brute_force_two_sum(&nums, target), [0, 1]);
    }

    #[test]
    fn two_sum_using_brute_force_for_invalid_list() {
        let nums = vec![2, 18, 13, 78];
        let target = 9;
        assert_eq!(brute_force_two_sum(&nums, target), []);
    }

    #[test]
    fn two_sum_using_brute_force_for_empty_list() {
        let nums = vec![];
        let target = 9;
        assert_eq!(brute_force_two_sum(&nums, target), []);
    }

    #[test]
    fn two_sum_using_hash_map_for_valid_list() {
        let nums = vec![2, 7, 13, 78];
        let target = 9;
        assert_eq!(hash_map_two_sum(&nums, target), [0, 1]);
    }
    #[test]
    fn two_sum_using_hash_map_for_invalid_list() {
        let nums = vec![2, 18, 13, 78];
        let target = 9;
        assert_eq!(hash_map_two_sum(&nums, target), []);
    }
    #[test]
    fn two_sum_using_hash_map_for_empty_list() {
        let nums = vec![];
        let target = 9;
        assert_eq!(hash_map_two_sum(&nums, target), []);
    }

    #[test]
    fn two_sum_using_optimized_hash_map_for_valid_list() {
        let nums = vec![2, 7, 13, 78];
        let target = 9;
        assert_eq!(optimized_hash_map_two_sum(&nums, target), [1, 0]);
    }
    #[test]
    fn two_sum_using_optimized_hash_map_for_invalid_list() {
        let nums = vec![2, 18, 13, 78];
        let target = 9;
        assert_eq!(optimized_hash_map_two_sum(&nums, target), []);
    }
    #[test]
    fn two_sum_using_optimized_hash_map_for_empty_list() {
        let nums = vec![];
        let target = 9;
        assert_eq!(optimized_hash_map_two_sum(&nums, target), []);
    }
}
