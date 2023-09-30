struct Solution;

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return true;
        }

        let mut direction = 0;

        for i in 1..nums.len() {
            if nums[i] > nums[i-1] {
                if direction == 0 {
                    direction = 1;
                } else if direction == -1 {
                    return false;
                }
            } else if nums[i] < nums[i-1] {
                if direction == 0 {
                    direction = -1;
                } else if direction == 1 {
                    return false;
                }
            }
        }
        true
    }
}
fn main() {
    assert_eq!(true, Solution::is_monotonic(vec![1,2,2,3,4]));
    assert_eq!(true, Solution::is_monotonic(vec![6,5,5,4,2]));
    assert_eq!(false, Solution::is_monotonic(vec![6,5,5,2,4]));
}
