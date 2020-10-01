fn main() {
    println!("{}", count(&vec![1, 2, 3, 4, 5, 10]));
}

fn count(list: &[i32]) -> i32 {
    if list.len() == 1 {
        return 1
    }

    1 + count(&list[..list.len() - 1])
}
