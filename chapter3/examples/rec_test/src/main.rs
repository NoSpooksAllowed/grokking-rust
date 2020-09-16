fn main() {
    countdown(3);
    fact(10);
}

fn countdown(i: i32) {
    println!("{}", i);

    if i <= 1 {
        return
    } else {
        countdown(i - 1)
    }
}

fn fact(x: i32) -> i32{
    if x == 1 {
        return 1;
    } else {
        return x * fact(x - 1)
    }
}
