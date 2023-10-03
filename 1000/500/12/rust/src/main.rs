use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut hm: HashMap<i32, i32> = HashMap::new();
        let mut ans = 0;

        for num in nums {
            if let Some(count) = hm.get(&num) {
                ans += count;
                hm.insert(num, count + 1);
            } else {
                hm.insert(num, 1);
            }
        }
        ans
    }
}

fn main() {
    assert_eq!(4, Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]));
    assert_eq!(6, Solution::num_identical_pairs(vec![1, 1, 1, 1]));
    assert_eq!(0, Solution::num_identical_pairs(vec![1, 2, 3]));
}
