fn bubble_sort(mut array: Vec<i32>) -> Vec<i32> {
    for i in 0..array.len() {
        for j in 0..array.len() - 1 - i {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
    array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_unsorted_array() {
        let arr = vec![5, 1, 4, 2, 8];

        let result = bubble_sort(arr);

        assert_eq!(result, vec![1, 2, 4, 5, 8]);
    }

    #[test]
    fn sorts_already_sorted_array() {
        let arr = vec![1, 2, 3, 4, 5];

        let result = bubble_sort(arr);

        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn sorts_reverse_sorted_array() {
        let arr = vec![5, 4, 3, 2, 1];

        let result = bubble_sort(arr);

        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn handles_duplicates() {
        let arr = vec![4, 2, 4, 1, 2];

        let result = bubble_sort(arr);

        assert_eq!(result, vec![1, 2, 2, 4, 4]);
    }

    #[test]
    fn handles_negative_numbers() {
        let arr = vec![-3, -1, -7, -2, -5];

        let result = bubble_sort(arr);

        assert_eq!(result, vec![-7, -5, -3, -2, -1]);
    }

    #[test]
    fn handles_mixed_positive_negative_and_zero() {
        let arr = vec![0, -10, 5, -3, 8, 2];

        let result = bubble_sort(arr);

        assert_eq!(result, vec![-10, -3, 0, 2, 5, 8]);
    }

    #[test]
    fn handles_single_element_array() {
        let arr = vec![42];

        let result = bubble_sort(arr);

        assert_eq!(result, vec![42]);
    }

    #[test]
    fn handles_empty_array() {
        let arr: Vec<i32> = vec![];

        let result = bubble_sort(arr);

        assert_eq!(result, vec![]);
    }

    #[test]
    fn handles_all_equal_elements() {
        let arr = vec![7, 7, 7, 7];

        let result = bubble_sort(arr);

        assert_eq!(result, vec![7, 7, 7, 7]);
    }
}
