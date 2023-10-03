struct Solution;

impl Solution {
    pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
        let mut seats = seats;
        let mut students = students;
        seats.sort();
        students.sort();
        seats
            .iter()
            .zip(students.iter())
            .map(|(seat, student)| (seat - student).abs())
            .sum()
    }
}

fn main() {
    assert_eq!(4, Solution::min_moves_to_seat(vec![3, 1, 5], vec![2, 7, 4]));
    assert_eq!(
        7,
        Solution::min_moves_to_seat(vec![4, 1, 5, 9], vec![1, 3, 2, 6])
    );
    assert_eq!(
        4,
        Solution::min_moves_to_seat(vec![2, 2, 6, 6], vec![1, 3, 2, 6])
    );
}
