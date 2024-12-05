fn main() {
    // read content from file input.txt
    let file_path = "input.txt"; // change this to the path of your file
    let contents = std::fs::read_to_string(file_path).expect("File Read error");

    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    for line in contents.lines() {
        let record = line.split("   ").collect::<Vec<_>>();
        left_list.push(record.get(0).unwrap().parse().unwrap());
        right_list.push(record.get(1).unwrap().parse().unwrap());
    }

    // 取得排序後的 index
    let left_sorted = argsort(&left_list);
    let right_sorted = argsort(&right_list);

    let mut result: u32 = 0;

    for i in 0..left_sorted.len() {
        result += left_list[left_sorted[i]].abs_diff(right_list[right_sorted[i]]);
    }

    print!("part1: {}", result);
}

/// Get the indices that sort a vector.
///
/// This is from https://stackoverflow.com/a/69764256
///
/// ## Example
///
/// ```rust
/// let v = vec![1, 7, 4, 2];
/// let i = argsort(&v);
/// assert_eq!(i, &[0, 3, 2, 1]);
/// ```
fn argsort<T: Ord>(data: &[T]) -> Vec<usize> {
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| &data[i]);
    indices
}

#[cfg(test)]
mod test {
    use super::argsort;

    #[test]
    fn test_argsort() {
        let v = vec![1, 7, 4, 2];
        let i = argsort(&v);
        assert_eq!(i, &[0, 3, 2, 1]);
    }

    #[test]
    fn test_argsort_with_example_left() {
        let v = vec![3, 4, 2, 1, 3, 3];
        let i = argsort(&v);
        assert_eq!(i, &[3, 2, 0, 4, 5, 1]);
    }

    #[test]
    fn test_argsort_with_example_right() {
        let v = vec![4, 3, 5, 3, 9, 3];
        let i = argsort(&v);
        assert_eq!(i, &[1, 3, 5, 0, 2, 4]);
    }
}
