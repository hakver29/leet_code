pub struct Solution {}


impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        (x as f64).sqrt() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::my_sqrt(4));
        assert_eq!(2, Solution::my_sqrt(8));
    }
}