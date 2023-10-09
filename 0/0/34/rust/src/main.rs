struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let i = nums.partition_point(|n| n < &target);

        if i == nums.len() || nums[i] != target {return vec![-1,-1];}

        let j = nums.partition_point(|n| n <= &target);
        vec![i as i32, (j-1) as i32]
    }
}
fn main() {
    assert_eq!(vec![3,4], Solution::search_range(vec![5,7,7,8,8,10], 8));
    assert_eq!(vec![-1,-1], Solution::search_range(vec![5,7,7,8,8,10], 6));
    assert_eq!(vec![-1,-1], Solution::search_range(vec![], 8));
}
