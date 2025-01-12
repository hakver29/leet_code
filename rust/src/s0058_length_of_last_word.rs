pub struct Solution;
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim_end().split(' ').last().unwrap().len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::length_of_last_word(String::from("Hello World")));
        assert_eq!(4, Solution::length_of_last_word(String::from("   fly me   to   the moon  ")));
        assert_eq!(6, Solution::length_of_last_word(String::from("luffy is still joyboy")));
    }
}