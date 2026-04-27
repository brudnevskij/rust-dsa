#[derive(Debug, PartialEq, Eq)]
enum InsertError {
    IndexOutOfCapacity,
    ArrayIsFull,
}

pub fn insert(
    values: &mut [i32],
    len: &mut usize,
    index: usize,
    value: i32,
) -> Result<(), InsertError> {
    if index > *len {
        println!("{}", values.len());
        return Err(InsertError::IndexOutOfCapacity);
    } else if *len == values.len() {
        return Err(InsertError::ArrayIsFull);
    }

    for i in (index + 1..=*len).rev() {
        values[i] = values[i - 1];
    }
    values[index] = value;
    *len += 1;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inserts_into_empty_array_at_beginning() {
        let mut values = [0; 5];
        let mut len = 0;

        let result = insert(&mut values, &mut len, 0, 10);

        assert_eq!(result, Ok(()));
        assert_eq!(len, 1);
        assert_eq!(&values[..len], &[10]);
    }

    #[test]
    fn inserts_at_beginning_and_shifts_values_right() {
        let mut values = [10, 20, 30, 0, 0];
        let mut len = 3;

        let result = insert(&mut values, &mut len, 0, 5);

        assert_eq!(result, Ok(()));
        assert_eq!(len, 4);
        assert_eq!(&values[..len], &[5, 10, 20, 30]);
    }

    #[test]
    fn inserts_at_end() {
        let mut values = [10, 20, 30, 0, 0];
        let mut len = 3;

        let result = insert(&mut values, &mut len, 3, 40);

        assert_eq!(result, Ok(()));
        assert_eq!(len, 4);
        assert_eq!(&values[..len], &[10, 20, 30, 40]);
    }

    #[test]
    fn inserts_in_middle_and_shifts_values_right() {
        let mut values = [10, 20, 30, 40, 0];
        let mut len = 4;

        let result = insert(&mut values, &mut len, 2, 99);

        assert_eq!(result, Ok(()));
        assert_eq!(len, 5);
        assert_eq!(&values[..len], &[10, 20, 99, 30, 40]);
    }

    #[test]
    fn returns_error_when_array_is_full() {
        let mut values = [10, 20, 30];
        let mut len = 3;

        let result = insert(&mut values, &mut len, 1, 99);

        assert_eq!(result, Err(InsertError::ArrayIsFull));
        assert_eq!(len, 3);
        assert_eq!(&values[..len], &[10, 20, 30]);
    }

    #[test]
    fn returns_error_when_index_is_greater_than_len() {
        let mut values = [10, 20, 30, 0, 0];
        let mut len = 3;

        let result = insert(&mut values, &mut len, 4, 99);

        assert_eq!(result, Err(InsertError::IndexOutOfCapacity));
        assert_eq!(len, 3);
        assert_eq!(&values[..len], &[10, 20, 30]);
    }

    #[test]
    fn allows_insert_at_index_equal_to_len() {
        let mut values = [10, 20, 30, 0, 0];
        let mut len = 3;

        let result = insert(&mut values, &mut len, 3, 40);

        assert_eq!(result, Ok(()));
        assert_eq!(len, 4);
        assert_eq!(&values[..len], &[10, 20, 30, 40]);
    }
}
