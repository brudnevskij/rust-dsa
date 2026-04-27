use std::collections::BTreeMap;

fn count_sort(values: Vec<i32>) -> Vec<i32> {
    let mut counts = BTreeMap::new();

    for value in values {
        *counts.entry(value).or_insert(0) += 1;
    }

    let mut result = Vec::new();
    for (value, count) in counts {
        for _ in 0..count {
            result.push(value);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_empty_vector() {
        let values = vec![];

        let sorted = count_sort(values);

        assert_eq!(sorted, vec![]);
    }

    #[test]
    fn sorts_single_element() {
        let values = vec![42];

        let sorted = count_sort(values);

        assert_eq!(sorted, vec![42]);
    }

    #[test]
    fn sorts_already_sorted_vector() {
        let values = vec![1, 2, 3, 4, 5];

        let sorted = count_sort(values);

        assert_eq!(sorted, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn sorts_reverse_sorted_vector() {
        let values = vec![5, 4, 3, 2, 1];

        let sorted = count_sort(values);

        assert_eq!(sorted, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn sorts_vector_with_duplicates() {
        let values = vec![4, 2, 2, 8, 3, 3, 1];

        let sorted = count_sort(values);

        assert_eq!(sorted, vec![1, 2, 2, 3, 3, 4, 8]);
    }

    #[test]
    fn sorts_vector_with_all_same_values() {
        let values = vec![7, 7, 7, 7];

        let sorted = count_sort(values);

        assert_eq!(sorted, vec![7, 7, 7, 7]);
    }

    #[test]
    fn sorts_vector_with_negative_numbers() {
        let values = vec![0, -5, 3, -1, -5, 2];

        let sorted = count_sort(values);

        assert_eq!(sorted, vec![-5, -5, -1, 0, 2, 3]);
    }

    #[test]
    fn sorts_vector_with_large_range() {
        let values = vec![1000, -1000, 0, 500, -500];

        let sorted = count_sort(values);

        assert_eq!(sorted, vec![-1000, -500, 0, 500, 1000]);
    }
}
