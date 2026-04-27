fn flip_array(mut values: Vec<i32>) -> Vec<i32> {
    if values.is_empty() {
        return values;
    }

    let mut left = 0;
    let mut right = values.len() - 1;
    while left < right {
        values.swap(left, right);
        left += 1;
        right -= 1;
    }

    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flips_empty_vector() {
        let values = vec![];

        let flipped = flip_array(values);

        assert_eq!(flipped, vec![]);
    }

    #[test]
    fn flips_single_element_vector() {
        let values = vec![42];

        let flipped = flip_array(values);

        assert_eq!(flipped, vec![42]);
    }

    #[test]
    fn flips_two_elements() {
        let values = vec![1, 2];

        let flipped = flip_array(values);

        assert_eq!(flipped, vec![2, 1]);
    }

    #[test]
    fn flips_odd_length_vector() {
        let values = vec![1, 2, 3, 4, 5];

        let flipped = flip_array(values);

        assert_eq!(flipped, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn flips_even_length_vector() {
        let values = vec![1, 2, 3, 4, 5, 6];

        let flipped = flip_array(values);

        assert_eq!(flipped, vec![6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn flips_vector_with_duplicates() {
        let values = vec![1, 2, 2, 3, 3];

        let flipped = flip_array(values);

        assert_eq!(flipped, vec![3, 3, 2, 2, 1]);
    }

    #[test]
    fn flips_vector_with_negative_numbers() {
        let values = vec![-3, -2, -1, 0, 1];

        let flipped = flip_array(values);

        assert_eq!(flipped, vec![1, 0, -1, -2, -3]);
    }
}
