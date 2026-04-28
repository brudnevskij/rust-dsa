pub fn rotate_left(values: &mut [i32], k: usize) {}

pub fn rotate_right(values: &mut [i32], k: usize) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_left_by_one() {
        let mut values = [1, 2, 3, 4, 5];

        rotate_left(&mut values, 1);

        assert_eq!(values, [2, 3, 4, 5, 1]);
    }

    #[test]
    fn rotate_left_by_two() {
        let mut values = [1, 2, 3, 4, 5];

        rotate_left(&mut values, 2);

        assert_eq!(values, [3, 4, 5, 1, 2]);
    }

    #[test]
    fn rotate_left_by_zero_does_nothing() {
        let mut values = [1, 2, 3, 4, 5];

        rotate_left(&mut values, 0);

        assert_eq!(values, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn rotate_left_by_len_does_nothing() {
        let mut values = [1, 2, 3, 4, 5];

        rotate_left(&mut values, 5);

        assert_eq!(values, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn rotate_left_by_more_than_len_wraps_around() {
        let mut values = [1, 2, 3, 4, 5];

        rotate_left(&mut values, 7);

        assert_eq!(values, [3, 4, 5, 1, 2]);
    }

    #[test]
    fn rotate_left_empty_array_does_nothing() {
        let mut values: [i32; 0] = [];

        rotate_left(&mut values, 3);

        assert_eq!(values, []);
    }

    #[test]
    fn rotate_left_single_element_does_nothing() {
        let mut values = [42];

        rotate_left(&mut values, 10);

        assert_eq!(values, [42]);
    }

    #[test]
    fn rotate_right_by_one() {
        let mut values = [1, 2, 3, 4, 5];

        rotate_right(&mut values, 1);

        assert_eq!(values, [5, 1, 2, 3, 4]);
    }

    #[test]
    fn rotate_right_by_two() {
        let mut values = [1, 2, 3, 4, 5];

        rotate_right(&mut values, 2);

        assert_eq!(values, [4, 5, 1, 2, 3]);
    }

    #[test]
    fn rotate_right_by_zero_does_nothing() {
        let mut values = [1, 2, 3, 4, 5];

        rotate_right(&mut values, 0);

        assert_eq!(values, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn rotate_right_by_len_does_nothing() {
        let mut values = [1, 2, 3, 4, 5];

        rotate_right(&mut values, 5);

        assert_eq!(values, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn rotate_right_by_more_than_len_wraps_around() {
        let mut values = [1, 2, 3, 4, 5];

        rotate_right(&mut values, 7);

        assert_eq!(values, [4, 5, 1, 2, 3]);
    }

    #[test]
    fn rotate_right_empty_array_does_nothing() {
        let mut values: [i32; 0] = [];

        rotate_right(&mut values, 3);

        assert_eq!(values, []);
    }

    #[test]
    fn rotate_right_single_element_does_nothing() {
        let mut values = [42];

        rotate_right(&mut values, 10);

        assert_eq!(values, [42]);
    }
}
