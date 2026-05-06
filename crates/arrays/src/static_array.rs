fn insert<const LEN: usize>(values: [i32; LEN], index: usize, value: i32) -> Vec<i32> {
    let new_len = LEN + 1;
    let mut result = vec![0; new_len];

    for i in 0..index {
        result[i] = values[i];
    }

    for i in (index + 1..new_len).rev() {
        result[i] = values[i - 1];
    }
    result[index] = value;
    result
}

fn push_front<const LEN: usize>(values: [i32; LEN], value: i32) -> Vec<i32> {
    insert(values, 0, value)
}

fn push<const LEN: usize>(values: [i32; LEN], value: i32) -> Vec<i32> {
    insert(values, LEN, value)
}

fn delete<const LEN: usize>(values: &mut [i32; LEN], index: usize) -> Vec<i32> {
    let new_len = LEN - 1;
    let mut result = vec![0; new_len];

    for i in 0..index {
        result[i] = values[i];
    }

    for i in index..new_len {
        result[i] = values[i + 1];
    }
    result
}

fn pop_front<const LEN: usize>(values: &mut [i32; LEN]) -> Vec<i32> {
    delete(values, 0)
}

fn pop<const LEN: usize>(values: &mut [i32; LEN]) -> Vec<i32> {
    delete(values, LEN - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---------- insert ----------

    #[test]
    fn insert_in_the_middle() {
        let values = [1, 2, 3];

        let result = insert(values, 1, 99);

        assert_eq!(result, vec![1, 99, 2, 3]);
    }

    #[test]
    fn insert_at_the_beginning() {
        let values = [1, 2, 3];

        let result = insert(values, 0, 99);

        assert_eq!(result, vec![99, 1, 2, 3]);
    }

    #[test]
    fn insert_at_the_end() {
        let values = [1, 2, 3];

        let result = insert(values, 3, 99);

        assert_eq!(result, vec![1, 2, 3, 99]);
    }

    #[test]
    fn insert_into_empty_array() {
        let values: [i32; 0] = [];

        let result = insert(values, 0, 42);

        assert_eq!(result, vec![42]);
    }

    #[test]
    #[should_panic]
    fn insert_panics_when_index_is_too_large() {
        let values = [1, 2, 3];

        insert(values, 4, 99);
    }

    // ---------- push_front ----------

    #[test]
    fn push_front_adds_value_to_beginning() {
        let values = [1, 2, 3];

        let result = push_front(values, 99);

        assert_eq!(result, vec![99, 1, 2, 3]);
    }

    #[test]
    fn push_front_empty_array() {
        let values: [i32; 0] = [];

        let result = push_front(values, 99);

        assert_eq!(result, vec![99]);
    }

    // ---------- push ----------

    #[test]
    fn push_adds_value_to_end() {
        let values = [1, 2, 3];

        let result = push(values, 99);

        assert_eq!(result, vec![1, 2, 3, 99]);
    }

    #[test]
    fn push_empty_array() {
        let values: [i32; 0] = [];

        let result = push(values, 99);

        assert_eq!(result, vec![99]);
    }

    // ---------- delete ----------

    #[test]
    fn delete_from_middle() {
        let mut values = [1, 2, 3, 4];

        let result = delete(&mut values, 1);

        assert_eq!(result, vec![1, 3, 4]);
    }

    #[test]
    fn delete_from_beginning() {
        let mut values = [1, 2, 3, 4];

        let result = delete(&mut values, 0);

        assert_eq!(result, vec![2, 3, 4]);
    }

    #[test]
    fn delete_from_end() {
        let mut values = [1, 2, 3, 4];

        let result = delete(&mut values, 3);

        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn delete_from_single_element_array() {
        let mut values = [42];

        let result = delete(&mut values, 0);

        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    #[should_panic]
    fn delete_panics_when_index_is_too_large() {
        let mut values = [1, 2, 3];

        delete(&mut values, 3);
    }

    // ---------- pop_front ----------

    #[test]
    fn pop_front_removes_first_element() {
        let mut values = [1, 2, 3];

        let result = pop_front(&mut values);

        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn pop_front_single_element_array() {
        let mut values = [42];

        let result = pop_front(&mut values);

        assert_eq!(result, Vec::<i32>::new());
    }

    // ---------- pop ----------

    #[test]
    fn pop_removes_last_element() {
        let mut values = [1, 2, 3];

        let result = pop(&mut values);

        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn pop_single_element_array() {
        let mut values = [42];

        let result = pop(&mut values);

        assert_eq!(result, Vec::<i32>::new());
    }
}
