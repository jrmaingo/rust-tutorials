// generates the nth fibonacci number
fn main() {
    for i in 1..10 {
        print!("{}, ", fib_rec(i));
    }
    println!("");
    for i in 1..10 {
        print!("{}, ", fib_itr(i));
    }
    println!("");
}

// recursively compute nth fibonacci number
fn fib_rec(nth: i32) -> i32 {
    match nth {
        0|1 => nth,
        _   => fib_rec(nth - 2) + fib_rec(nth - 1)
    }
}

// iteratively compute nth fibonacci number
fn fib_itr(nth: i32) -> i32 {
    let mut last_pair = (0, 1);
    for _ in 1..nth {
        let tmp = last_pair.0 + last_pair.1;
        last_pair.0 = last_pair.1;
        last_pair.1 = tmp;
    }
    if nth == 0 { last_pair.0 } else { last_pair.1 }
}
