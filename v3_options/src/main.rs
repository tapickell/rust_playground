#[derive(Debug)]
pub enum Res<T, E> {
    Thing(T),
    Error(E),
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Result::Err("Cannot Divide by Zero".to_string());
    }
    Result::Ok(a / b)
}

fn handle_result(result: Result<i32, String>) {
    match result {
        Result::Ok(v) => println!("Value {}", v),
        Result::Err(s) => println!("Error {}", s),
    }
}

fn main() {
    let a = divide(10, 5);
    let b = divide(10, 0);

    handle_result(a);
    handle_result(b);

    // println!("{:?}, {:?}!", a, b);
}
