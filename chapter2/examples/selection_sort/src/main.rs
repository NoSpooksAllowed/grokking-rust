use std::cmp::PartialEq;
use std::cmp::PartialOrd;

fn main() {
    println!("{:?}", selection_sort(&mut vec![5, 3, 6, 2, 10]));
}

fn find_smallest<T:PartialEq + PartialOrd>(v: &Vec<T>) -> usize {
    let mut smallest = &v[0];
    let mut smallest_index = 0;

    for i in 1..v.len() {
        if v[i] < *smallest {
            smallest = &v[i];
            smallest_index = i;
        }
    }
    smallest_index
}

fn selection_sort<T:PartialEq + PartialOrd>(v: &mut Vec<T>) -> Vec<T> {
    let mut new_v = Vec::new();

    for _i in 0..v.len() {
        let smallest = find_smallest(&v);
        new_v.push(v.remove(smallest))
    }

    new_v
}
