use std::collections::{HashMap, HashSet};
fn main() {
    let s1 = "bcabc".to_string();
    let sol = Solution::remove_duplicate_letters(s1);
    println!("{sol}");
}

struct Solution;
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut stack: Vec<char> = Vec::new();
        let mut seen: HashSet<char> = HashSet::new();
        let mut last_occ: HashMap<char, usize> = HashMap::new();
        for (i, c) in s.chars().enumerate() {
            last_occ.insert(c, i);
        }

        for (i, c) in s.chars().enumerate() {
            if !seen.contains(&c) {
                while let Some(&top) = stack.last() {
                    if c < top && i < *last_occ.get(&top).unwrap() {
                        seen.remove(&stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                seen.insert(c);
                stack.push(c);
            }
        }
        stack.into_iter().collect()
    }
}