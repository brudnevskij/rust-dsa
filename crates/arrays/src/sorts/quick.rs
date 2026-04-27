fn quick_sort(values: Vec<i32>) -> Vec<i32> {
    if values.is_empty() {
        return vec![];
    }

    let pivot = values[0];

    let (less_or_equal, greater): (Vec<_>, Vec<_>) =
        values.into_iter().skip(1).partition(|n| *n <= pivot);

    let mut sorted = quick_sort(less_or_equal);
    let greater = quick_sort(greater);

    sorted.push(pivot);
    sorted.extend(greater);

    sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_empty_array() {
        let input = vec![];
        let result = quick_sort(input);

        assert_eq!(result, vec![]);
    }

    #[test]
    fn sorts_single_element_array() {
        let input = vec![42];
        let result = quick_sort(input);

        assert_eq!(result, vec![42]);
    }

    #[test]
    fn sorts_already_sorted_array() {
        let input = vec![1, 2, 3, 4, 5];
        let result = quick_sort(input);

        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn sorts_reverse_sorted_array() {
        let input = vec![5, 4, 3, 2, 1];
        let result = quick_sort(input);

        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn sorts_unsorted_array() {
        let input = vec![3, 1, 4, 5, 2];
        let result = quick_sort(input);

        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn sorts_array_with_duplicates() {
        let input = vec![4, 2, 4, 1, 3, 2];
        let result = quick_sort(input);

        assert_eq!(result, vec![1, 2, 2, 3, 4, 4]);
    }

    #[test]
    fn sorts_array_with_negative_numbers() {
        let input = vec![3, -1, 0, -5, 2];
        let result = quick_sort(input);

        assert_eq!(result, vec![-5, -1, 0, 2, 3]);
    }

    #[test]
    fn sorts_array_with_all_equal_elements() {
        let input = vec![7, 7, 7, 7];
        let result = quick_sort(input);

        assert_eq!(result, vec![7, 7, 7, 7]);
    }

    #[test]
    fn sorts_large_mixed_array() {
        let input = vec![10, -3, 5, 0, 8, 8, -10, 2, 1];
        let result = quick_sort(input);

        assert_eq!(result, vec![-10, -3, 0, 1, 2, 5, 8, 8, 10]);
    }
}
