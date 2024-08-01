// N entries on stack
fn fact(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }
    return n * fact(n - 1);
}

fn factorial_tail(n: i32) -> i32 {
    return fact_tail(n, 1);
}

// Doesnt build up the stack because the calculations are done on the way up
// instead of on the return call
fn fact_tail(n: i32, r: i32) -> i32 {
    if n <= 1 {
        return r;
    }
    return fact_tail(n - 1, n * r);
}

fn main() {
    println!("fact {}", fact(9));
    println!("fact_tail {}", factorial_tail(9));
}
