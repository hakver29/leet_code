pub struct Solution {}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut increment = digits.clone();

        let mut index = digits.len() - 1;

        if digits[index] == 9 {
            while digits[index] == 9 {
                increment[index] = 0;
                if index == 0 {
                    if increment[index] == 0 {
                        increment.insert(0, 1);
                    } else {
                        increment[index] += 1;
                    }
                    return increment;
                }
                index -= 1;
            }
            increment[index] += 1;
            increment
        } else{
            increment[index] += 1;
            increment
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![1,2,4], Solution::plus_one(vec![1, 2, 3]));
        assert_eq!(vec![4,3,2,2], Solution::plus_one(vec![4, 3, 2, 1]));
        assert_eq!(vec![1, 0], Solution::plus_one(vec![9]));
        assert_eq!(vec![9,0,0,0], Solution::plus_one(vec![8,9,9,9]));

    }
}
