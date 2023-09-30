use std::vec;

struct Solution;

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
    let mut stack: Vec<i32> = Vec::new();
    let mut third = i32::MIN;

    for &num in nums.iter().rev() {
        if num < third {
            return true;
        }
        while let Some(&top) = stack.last() {
            if top < num {
                third = stack.pop().unwrap();
            } else {
                break;
            }
        }
        stack.push(num);
    }
    false
}
}

fn main() {
    assert_eq!(false, Solution::find132pattern(vec![4,3,2,1]));
    //assert_eq!(true, Solution::find132pattern(vec![1,3,2, 4]));
    //assert_eq!(true, Solution::find132pattern(vec![-1,3,2, 0]));
}
