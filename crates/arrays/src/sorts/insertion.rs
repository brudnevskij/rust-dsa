fn insertion_sort(mut values: Vec<i32>) -> Vec<i32> {
    for i in 1..values.len() {
        let mut j = i;

        while j > 0 && values[j - 1] > values[j] {
            values.swap(j, j - 1);
            j -= 1;
        }
    }

    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_empty_vector() {
        let input = vec![];

        let result = insertion_sort(input);

        assert_eq!(result, vec![]);
    }

    #[test]
    fn sorts_single_element_vector() {
        let input = vec![42];

        let result = insertion_sort(input);

        assert_eq!(result, vec![42]);
    }

    #[test]
    fn sorts_already_sorted_vector() {
        let input = vec![1, 2, 3, 4, 5];

        let result = insertion_sort(input);

        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn sorts_reverse_sorted_vector() {
        let input = vec![5, 4, 3, 2, 1];

        let result = insertion_sort(input);

        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn sorts_unsorted_vector() {
        let input = vec![4, 2, 5, 1, 3];

        let result = insertion_sort(input);

        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn sorts_vector_with_duplicates() {
        let input = vec![3, 1, 2, 3, 2, 1];

        let result = insertion_sort(input);

        assert_eq!(result, vec![1, 1, 2, 2, 3, 3]);
    }

    #[test]
    fn sorts_vector_with_negative_numbers() {
        let input = vec![3, -1, 0, -5, 2];

        let result = insertion_sort(input);

        assert_eq!(result, vec![-5, -1, 0, 2, 3]);
    }

    #[test]
    fn sorts_vector_with_all_equal_elements() {
        let input = vec![7, 7, 7, 7];

        let result = insertion_sort(input);

        assert_eq!(result, vec![7, 7, 7, 7]);
    }

    #[test]
    fn matches_builtin_sort() {
        let input = vec![9, 1, 5, 3, 7, 3, 0, -2, 8];

        let mut expected = input.clone();
        expected.sort();

        let result = insertion_sort(input);

        assert_eq!(result, expected);
    }
}
