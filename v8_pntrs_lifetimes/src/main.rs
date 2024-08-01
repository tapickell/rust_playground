#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    age: i32,
}

impl Person {
    pub fn new(name: String, age: i32) -> Self {
        Person { name, age }
    }

    // self and mut self take ownership of the object and never give it back
    // unless made return type of function
    // &self borrow I want to read only
    // &mut self borrow as mutable and return it
    // pub fn print(self, &self, &mut self, mut self) {
    // you usually want to borrow an object
    pub fn greet(&self) -> String {
        format!("Hi my name is {}", self.name)
    }

    pub fn age_update(&mut self, n: i32) {
        self.age += n;
    }

    pub fn dropme(self) {}
}

pub fn get_age(s: &Person) -> &i32 {
    &s.age
}

fn main() {
    let p = Person::new("Todd".to_string(), 42);
    // p.age_update(24); cannot borrow as mutable
    let mut p2 = Person::new("Todd Pickell".to_string(), 42);
    p2.age_update(23);
    let age = get_age(&p2);
    // p2.age_update(21); cannot borrow as mutable already borrowed as immutable

    // p2.dropme(); // consumes p2 so it is no longer available
    // p2.greet(); value borrowed here after move

    println!("{}", age);
    println!("Hello, world!, {}", p.greet());
    let greeting = p.greet();
    println!("Hello, world!, {}", greeting);
}
