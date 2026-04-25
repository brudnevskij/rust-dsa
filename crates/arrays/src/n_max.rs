fn n_maximums(array: &[i64], max_quantity: usize) -> Vec<i64> {
    assert!(array.len() >= max_quantity);

    let mut used_indexes = vec![false; array.len()];
    let mut result = Vec::with_capacity(max_quantity);

    for _ in 0..max_quantity {
        let mut max_idx: Option<usize> = None;

        for i in 0..array.len() {
            // ignore used maximums
            if used_indexes[i] {
                continue;
            }

            match max_idx {
                // if no index assigned
                // assign first unused one
                None => max_idx = Some(i),
                // otherwise compare two indexes
                Some(j) => {
                    if array[j] < array[i] {
                        max_idx = Some(i);
                    }
                }
            }
        }

        let max_idx = max_idx.unwrap();
        used_indexes[max_idx] = true;
        result.push(array[max_idx]);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_one_maximum() {
        let arr = [1, 5, 3, 2];

        let result = n_maximums(&arr, 1);

        assert_eq!(result, vec![5]);
    }

    #[test]
    fn returns_n_maximums_in_descending_order() {
        let arr = [5, 1, 9, 3, 7, 2];

        let result = n_maximums(&arr, 3);

        assert_eq!(result, vec![9, 7, 5]);
    }

    #[test]
    fn handles_duplicates() {
        let arr = [5, 1, 9, 3, 9, 2, 7];

        let result = n_maximums(&arr, 3);

        assert_eq!(result, vec![9, 9, 7]);
    }

    #[test]
    fn handles_all_equal_elements() {
        let arr = [4, 4, 4, 4];

        let result = n_maximums(&arr, 2);

        assert_eq!(result, vec![4, 4]);
    }

    #[test]
    fn handles_negative_numbers() {
        let arr = [-10, -3, -7, -1, -20];

        let result = n_maximums(&arr, 2);

        assert_eq!(result, vec![-1, -3]);
    }

    #[test]
    fn handles_mixed_positive_and_negative_numbers() {
        let arr = [-10, 5, 0, -3, 8, 2];

        let result = n_maximums(&arr, 4);

        assert_eq!(result, vec![8, 5, 2, 0]);
    }

    #[test]
    fn when_n_equals_array_length_returns_all_elements_descending() {
        let arr = [3, 1, 4, 2];

        let result = n_maximums(&arr, 4);

        assert_eq!(result, vec![4, 3, 2, 1]);
    }

    #[test]
    fn handles_single_element_array() {
        let arr = [42];

        let result = n_maximums(&arr, 1);

        assert_eq!(result, vec![42]);
    }

    #[test]
    fn does_not_modify_original_array() {
        let arr = [5, 1, 9, 3];
        let original = arr;

        let _ = n_maximums(&arr, 2);

        assert_eq!(arr, original);
    }
}
