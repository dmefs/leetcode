struct Solution;

impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        let mut length : i64 = 0;
        let bytes = s.as_bytes();
        let mut i = 0;
        let mut k = k;

        while length < k as i64 {
            if bytes[i].is_ascii_digit() {
                length *= (bytes[i] as char).to_digit(10).unwrap() as i64;
            } else {
                length += 1;
            }
            i += 1;
        }

        for j in (0..i).rev() {
            if bytes[j].is_ascii_digit() {
                length /= (bytes[j] as char).to_digit(10).unwrap() as i64;
                k = ((k as i64)  % length) as i32;
            } else {
                if k == 0 || k as i64 == length {
                    return (bytes[j] as char).to_string();
                }
                length -= 1;
            }
        }
        "".to_string()
    }
}
fn main() {
    assert_eq!("o".to_string(), Solution::decode_at_index("leet2code3".to_string(), 10));
    assert_eq!("h".to_string(), Solution::decode_at_index("ha22".to_string(), 5));
    assert_eq!("a".to_string(), Solution::decode_at_index("a2345678999999999999999".to_string(), 1));
}
