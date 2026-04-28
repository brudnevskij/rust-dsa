fn move_zeroes(values: &mut [i32]) {
    let mut write_idx = 0;

    for read_idx in 0..values.len() {
        if values[read_idx] != 0 {
            values[write_idx] = values[read_idx];
            write_idx += 1;
        }
    }

    for value in values.iter_mut().skip(write_idx) {
        *value = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moves_zeroes_to_the_end() {
        let mut values = [0, 1, 0, 3, 12];

        move_zeroes(&mut values);

        assert_eq!(values, [1, 3, 12, 0, 0]);
    }

    #[test]
    fn keeps_order_of_non_zero_elements() {
        let mut values = [4, 0, 5, 0, 1, 0, 3];

        move_zeroes(&mut values);

        assert_eq!(values, [4, 5, 1, 3, 0, 0, 0]);
    }

    #[test]
    fn array_without_zeroes_stays_the_same() {
        let mut values = [1, 2, 3, 4];

        move_zeroes(&mut values);

        assert_eq!(values, [1, 2, 3, 4]);
    }

    #[test]
    fn array_with_only_zeroes_stays_the_same() {
        let mut values = [0, 0, 0, 0];

        move_zeroes(&mut values);

        assert_eq!(values, [0, 0, 0, 0]);
    }

    #[test]
    fn empty_array_stays_empty() {
        let mut values: [i32; 0] = [];

        move_zeroes(&mut values);

        assert_eq!(values, []);
    }

    #[test]
    fn single_zero_stays_the_same() {
        let mut values = [0];

        move_zeroes(&mut values);

        assert_eq!(values, [0]);
    }

    #[test]
    fn single_non_zero_stays_the_same() {
        let mut values = [7];

        move_zeroes(&mut values);

        assert_eq!(values, [7]);
    }

    #[test]
    fn zeroes_at_the_end_stay_at_the_end() {
        let mut values = [1, 2, 3, 0, 0];

        move_zeroes(&mut values);

        assert_eq!(values, [1, 2, 3, 0, 0]);
    }

    #[test]
    fn zeroes_at_the_beginning_move_to_the_end() {
        let mut values = [0, 0, 1, 2, 3];

        move_zeroes(&mut values);

        assert_eq!(values, [1, 2, 3, 0, 0]);
    }

    #[test]
    fn works_with_negative_numbers() {
        let mut values = [0, -1, 0, -3, 2];

        move_zeroes(&mut values);

        assert_eq!(values, [-1, -3, 2, 0, 0]);
    }
}
