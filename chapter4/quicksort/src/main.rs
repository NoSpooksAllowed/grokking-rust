use std::cmp::PartialOrd;

fn main() {
}

fn quicksort<T:PartialOrd>(v: &mut [T]) {
    let len = v.len();
    if len >= 2 {
        let pivot_index = partition(v);
        quicksort(&mut v[0..pivot_index]);
        quicksort(&mut v[pivot_index + 1..len]);
    }
}

fn partition<T:PartialOrd>(v: &mut [T]) -> usize {
    let len = v.len();
    let pivot_index = len / 2;
    let last_index = len - 1;

    v.swap(pivot_index, last_index);

    let mut store_index = 0;
    for i in 0..last_index {
        if v[i] < v[last_index] {
            v.swap(i, store_index);
            store_index += 1;
        }
    }

    v.swap(store_index, len - 1);
    store_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quicksort() {
        let mut v = vec![10, 5, 6, 14, 16, 9, 100, 15, 8];
        quicksort(&mut v);
        assert_eq!(vec![5, 6, 8, 9, 10, 14, 15, 16, 100], v);
    }
}
