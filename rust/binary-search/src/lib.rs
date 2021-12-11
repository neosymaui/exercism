pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut start = 0 as usize;
    let mut end = array.len() as usize;

    while start <= end {
        let mid = (start + end) / 2 as usize;

        if array.len() <= mid {
            return None;
        }

        match (key - array[mid]) as i32 {
            v if v > 0 => start = mid + 1,
            v if v < 0 => {
                if mid == 0 {
                    return None;
                }
                end = mid - 1;
            }
            _ => return Some(mid),
        }
    }
    None
}
