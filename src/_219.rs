/// 219. Contains Duplicate II
///
/// Given an integer array nums and an integer k,
/// return true if there are two distinct indices
/// i and j in the array such that nums[i] == nums[j]
/// and abs(i - j) <= k.
///
/// Constraints:
/// * 1 <= nums.length <= 10^5
/// * -10^9 <= nums[i] <= 10^9
/// * 0 <= k <= 10^5

fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    // Store number-to-index mappings in a HashMap.
    // Look for two equal numbers, one current, one stored.
    // Check for success condition.
    // Time complexity: O(n).
    //  - n is the length of the nums array.
    //  - Looks at each element in the nums array.
    // Space complexity: O(n).
    //  - n is the length of the nums array.
    //  - Stores each element from the nums array in a HashMap.
    use std::collections::HashMap;

    // Contributes to O(n) space complexity.
    let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());

    // Loop through each element in the nums array.
    // Contributes to O(n) time complexity.
    for (idx_num, num) in nums.into_iter().enumerate() {
        // Look for previously seen number.
        if let Some(&idx_map) = map.get(&num) {
            // Check for success condition.
            // idx_num always larger than idx_map.
            if idx_num - idx_map <= k as usize {
                return true;
            }
        }
        // Store new number-index mapping.
        map.insert(num, idx_num);
    }

    false
}

fn contains_nearby_duplicate_d(nums: Vec<i32>, k: i32) -> bool {
    // Use a HashMap to store number-to-index mappings.
    // Check success condition when both numbers found.
    // Time complexity: O(n).
    //  - n is the length of the nums array.
    //  - Look at each element once.
    // Space complexity: O(n).
    //  - n is the length of the nums array.
    //  - Store each element of the nums array.
    use std::collections::HashMap;

    // Contributes O(n) space complexity.
    let mut map: HashMap<i32, usize> = HashMap::new();

    // Loop through each element of the nums array.
    // Contributes O(n) time complexity.
    for (idx_num, num) in nums.into_iter().enumerate() {
        // Check for previous number stored.
        if let Some(&idx_map) = map.get(&num) {
            // Check for success condition.
            // idx_map always smaller than idx_num.
            if idx_num - idx_map <= k as usize {
                return true;
            }
        }

        map.insert(num, idx_num);
    }

    false
}

fn contains_nearby_duplicate_c(nums: Vec<i32>, k: i32) -> bool {
    // Store a number to index mapping in a HashMap.
    // Loop through each number.
    // Look for two number which satisfy the success condition.
    // Time complexity: O(n).
    //  - n is the length of the nums array.
    // Space complexity: O(n).
    //  - n is the length of the nums array.
    //  - Possible to store up to n-1 elements in the HashMap.

    use std::collections::HashMap;

    let mut map: HashMap<i32, usize> = HashMap::new();

    for (idx_num, num) in nums.iter().enumerate() {
        if let Some(idx_map) = map.get(num) {
            if idx_num - idx_map <= k as usize {
                return true;
            }
        }

        map.insert(*num, idx_num);
    }

    false
}

fn contains_nearby_duplicate_b(nums: Vec<i32>, k: i32) -> bool {
    use std::collections::HashMap;

    // Map number to index.
    // Contributes to O(n) space complexity.
    let mut map = HashMap::new();

    // Loop through each element in `nums`.
    // Contributes to O(n) time complexity.
    for (idx_cur, num) in nums.iter().enumerate() {
        if let Some(&idx_map) = map.get(&num) {
            // Check success condition.
            // idx_cur is larger than idx_map.
            if idx_cur - idx_map <= k as usize {
                return true;
            }
        }

        // Insert current number and index.
        map.insert(num, idx_cur);
    }

    false
}

fn contains_nearby_duplicate_a(nums: Vec<i32>, k: i32) -> bool {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        if let Some(&j) = map.get(&num) {
            if i - j <= k as usize {
                return true;
            }
        }
        map.insert(num, i);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_true() {
        assert!(contains_nearby_duplicate(vec![1, 2, 3, 1], 3));
    }

    #[test]
    fn test_basic_false() {
        assert!(!contains_nearby_duplicate(vec![1, 2, 3, 4], 2));
    }

    #[test]
    fn test_edge_at_limit() {
        assert!(contains_nearby_duplicate(vec![1, 0, 1, 1], 1));
    }

    #[test]
    fn test_no_duplicates() {
        assert!(!contains_nearby_duplicate(vec![1, 2, 3, 4], 3));
    }

    #[test]
    fn test_large_k() {
        assert!(contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 5));
    }

    #[test]
    fn test_empty_input() {
        assert!(!contains_nearby_duplicate(vec![], 0));
    }
}
