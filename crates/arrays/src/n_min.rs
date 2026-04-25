use std::usize;

fn n_minimums(array: &[i32], n: usize) -> Vec<i32> {
    assert!(array.len() >= n);

    let mut used_indexes = vec![false; array.len()];
    let mut result = Vec::with_capacity(n);

    for _ in 0..n {
        let mut minimum_idx: Option<usize> = None;

        for i in 0..array.len() {
            if used_indexes[i] {
                continue;
            }

            match minimum_idx {
                None => minimum_idx = Some(i),
                Some(j) => {
                    if array[i] < array[j] {
                        minimum_idx = Some(i)
                    }
                }
            }
        }

        let minimum_idx = minimum_idx.unwrap();
        used_indexes[minimum_idx] = true;
        result.push(array[minimum_idx]);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_one_minimum() {
        let arr = [4, 2, 7, 1, 9];

        let result = n_minimums(&arr, 1);

        assert_eq!(result, vec![1]);
    }

    #[test]
    fn returns_n_minimums_in_ascending_order() {
        let arr = [5, 1, 9, 3, 7, 2];

        let result = n_minimums(&arr, 3);

        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn handles_duplicates() {
        let arr = [5, 1, 1, 3, 7, 2];

        let result = n_minimums(&arr, 3);

        assert_eq!(result, vec![1, 1, 2]);
    }

    #[test]
    fn handles_all_equal_elements() {
        let arr = [4, 4, 4, 4];

        let result = n_minimums(&arr, 2);

        assert_eq!(result, vec![4, 4]);
    }

    #[test]
    fn handles_negative_numbers() {
        let arr = [-10, -3, -7, -1, -20];

        let result = n_minimums(&arr, 2);

        assert_eq!(result, vec![-20, -10]);
    }

    #[test]
    fn handles_mixed_positive_and_negative_numbers() {
        let arr = [-10, 5, 0, -3, 8, 2];

        let result = n_minimums(&arr, 4);

        assert_eq!(result, vec![-10, -3, 0, 2]);
    }

    #[test]
    fn when_n_equals_array_length_returns_all_elements_ascending() {
        let arr = [3, 1, 4, 2];

        let result = n_minimums(&arr, 4);

        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn handles_single_element_array() {
        let arr = [42];

        let result = n_minimums(&arr, 1);

        assert_eq!(result, vec![42]);
    }

    #[test]
    fn does_not_modify_original_array() {
        let arr = [5, 1, 9, 3];
        let original = arr;

        let _ = n_minimums(&arr, 2);

        assert_eq!(arr, original);
    }

    #[test]
    fn handles_zero() {
        let arr = [10, 0, -1, 5, 3];

        let result = n_minimums(&arr, 3);

        assert_eq!(result, vec![-1, 0, 3]);
    }
}
