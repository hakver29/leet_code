pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        for i in nums.iter() {
            if nums.iter().filter(|&n| *n == *i).count() == 1 {
                return *i;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::single_number(vec![2,2,1]));
        assert_eq!(4, Solution::single_number(vec![4,1,2,1,2]));
        assert_eq!(1, Solution::single_number(vec![1]));
    }
}