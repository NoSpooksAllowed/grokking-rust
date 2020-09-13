use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use text_io::read;

fn main() {
    let numbers = vec![1, 3, 5, 7, 9];

    println!("Hello, please enter a number you want to find: ");
    let num: i32 = read!();

    println!("In the array {:?}", numbers);

    match binary_search(&numbers, num) {
        Some(x) => println!("Number {} in the array has index {}", num, x),
        None => println!("There is no {} number in this array", num),
    }
}

fn binary_search<T:PartialEq + PartialOrd>(list: &Vec<T>, item: T) -> Option<i32> {
    let mut low = 0;
    let mut high = (list.len() - 1) as i32;

    while low <= high {
        let mid = (low + high) / 2;
        let guess = &list[mid as usize];
        
        if *guess == item {
            return Some(mid) ;
        } if *guess > item {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    None
}
