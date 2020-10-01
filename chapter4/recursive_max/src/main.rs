use std::cmp;

fn main() {
    println!("{}", max_in_vector(&vec![10, 5, 7, 9, 15, 6, 11, 8, 12, 2]));
}

fn max_in_vector(list: &[i32]) -> i32 {
    if list.len() == 1 {
        return list[0]
    }

    cmp::max(max_in_vector(&list[..list.len() - 1]), list[list.len() - 1])
}
