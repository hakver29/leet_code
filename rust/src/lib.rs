mod s0035_search_insert_position;
mod s0001_two_sum;
mod s0058_length_of_last_word;
mod s0066_plus_one;
mod s0136_single_number;
mod s0069_sqrt;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
