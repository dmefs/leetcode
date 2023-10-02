struct Solution;

impl Solution {
    pub fn winner_of_game(colors: String) -> bool {
        let mut count_a = 0;
        let mut count_b = 0;
        let mut count = 0;
        let mut v: Vec<char> = colors.chars().collect();
        v.push('C');

        for i in 1..v.len() {
            if v[i] == v[i - 1] {
                count += 1;
            } else if v[i - 1] == 'A' {
                count_a += (count - 1).max(0);
                count = 0;
            } else {
                count_b += (count - 1).max(0);
                count = 0;
            }
        }
        count_a > count_b
    }
}

fn main() {
    assert_eq!(true, Solution::winner_of_game("AAABABB".to_string()));
    assert_eq!(false, Solution::winner_of_game("AA".to_string()));
    assert_eq!(false, Solution::winner_of_game("ABBBBBBBAAA".to_string()));
}
