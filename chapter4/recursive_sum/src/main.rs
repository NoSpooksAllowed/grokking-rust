fn main() {
    println!("{}", sum(&vec![1, 2, 3, 4]));
}

fn sum(list: &[i32]) -> i32 {

    if list.len() == 1 {
        return list[0]
    }

    list[list.len() - 1] + sum(&list[..list.len() - 1])
}
