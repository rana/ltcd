pub fn remove_element(nums: &mut [i32], val: i32) -> i32 {
    // Check for elements which don't equal `val`
    // The first `k` elements are to meet th condition.
    // Use a two-pointer technique.
    // Return the number of elements which meet the condition.

    let mut lft: usize = 0;

    for rht in 0..nums.len() {
        if nums[rht] != val {
            nums[lft] = nums[rht];
            lft += 1;
        }
    }

    lft as i32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_element_basic_case() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        let new_length = remove_element(&mut nums, val);
        assert_eq!(new_length, 2);
        assert_eq!(&nums[..new_length as usize], &[2, 2]);
    }

    #[test]
    fn test_remove_element_no_removals() {
        let mut nums = vec![1, 2, 3, 4, 5];
        let val = 6;
        let new_length = remove_element(&mut nums, val);
        assert_eq!(new_length, 5);
        assert_eq!(&nums[..new_length as usize], &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_remove_element_all_removals() {
        let mut nums = vec![4, 4, 4, 4];
        let val = 4;
        let new_length = remove_element(&mut nums, val);
        assert_eq!(new_length, 0);
    }

    #[test]
    fn test_remove_element_mixed_values() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;
        let new_length = remove_element(&mut nums, val);
        assert_eq!(new_length, 5);
        // The order of the remaining elements may vary, so we just check for length and valid elements.
        let expected_elements: Vec<i32> = vec![0, 1, 3, 0, 4];
        let result: Vec<i32> = nums[..new_length as usize].to_vec();
        for &element in &result {
            assert!(expected_elements.contains(&element));
        }
    }

    #[test]
    fn test_remove_element_empty_array() {
        let mut nums: Vec<i32> = vec![];
        let val = 1;
        let new_length = remove_element(&mut nums, val);
        assert_eq!(new_length, 0);
    }

    #[test]
    fn test_remove_element_single_element_match() {
        let mut nums = vec![1];
        let val = 1;
        let new_length = remove_element(&mut nums, val);
        assert_eq!(new_length, 0);
    }

    #[test]
    fn test_remove_element_single_element_no_match() {
        let mut nums = vec![1];
        let val = 2;
        let new_length = remove_element(&mut nums, val);
        assert_eq!(new_length, 1);
        assert_eq!(&nums[..new_length as usize], &[1]);
    }
}
