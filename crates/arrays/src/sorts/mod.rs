mod bubble;
mod count;
mod insertion;
mod quick;
mod selection;

fn is_sorted<F: Ord>(values: &[F]) -> bool {
    values.windows(2).all(|w| w[0] <= w[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_array_is_sorted() {
        let values: [i32; 0] = [];

        assert!(is_sorted(&values));
    }

    #[test]
    fn single_element_array_is_sorted() {
        let values = [42];

        assert!(is_sorted(&values));
    }

    #[test]
    fn sorted_ascending_array_returns_true() {
        let values = [1, 2, 3, 4, 5];

        assert!(is_sorted(&values));
    }

    #[test]
    fn sorted_array_with_duplicates_returns_true() {
        let values = [1, 2, 2, 3, 3, 4];

        assert!(is_sorted(&values));
    }

    #[test]
    fn unsorted_array_returns_false() {
        let values = [1, 2, 5, 3, 4];

        assert!(!is_sorted(&values));
    }

    #[test]
    fn descending_array_returns_false() {
        let values = [5, 4, 3, 2, 1];

        assert!(!is_sorted(&values));
    }

    #[test]
    fn negative_numbers_sorted_returns_true() {
        let values = [-5, -3, -1, 0, 2];

        assert!(is_sorted(&values));
    }

    #[test]
    fn negative_numbers_unsorted_returns_false() {
        let values = [-5, -1, -3, 0, 2];

        assert!(!is_sorted(&values));
    }

    #[test]
    fn works_with_strings() {
        let values = ["apple", "banana", "banana", "orange"];

        assert!(is_sorted(&values));
    }

    #[test]
    fn unsorted_strings_return_false() {
        let values = ["banana", "apple", "orange"];

        assert!(!is_sorted(&values));
    }
}
