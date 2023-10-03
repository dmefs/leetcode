use solution::Solution;

mod solution;

fn main() {
    let words = vec![
        "a".to_string(),
        "b".to_string(),
        "ba".to_string(),
        "bca".to_string(),
        "bda".to_string(),
        "bdca".to_string(),
    ];
    let ans = Solution::longest_str_chain(words);
    println!("{ans}");
}
