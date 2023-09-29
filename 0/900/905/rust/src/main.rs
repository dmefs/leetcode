use std::vec;

struct Solution;

impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut i = 0;
        let mut j = nums.len() - 1;

        while i < j {
            while i < j && nums[i] % 2 == 0 {
                i += 1;
            }
            while i < j && nums[j] % 2 != 0 {
                j -= 1;
            }
            nums.swap(i, j);
        }
        nums
    }
}
fn main() {
    let v1 = vec![3,1,2,4];
    let ans = Solution::sort_array_by_parity(v1);
    assert_eq!(vec![4,2,1,3], ans);
}
