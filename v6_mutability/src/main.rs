// Copy may not be implemented for this type
// Because String is not Copy able
#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    age: i32,
}

// Must derive Clone before Copy
// Because all parts impl Copy we can derive Copy
#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

fn main() {
    let mut x = 34;
    let y = x; // copy
    x += 5; // mutation

    println!("{}, {}", y, x);

    let mut p = Person {
        name: "Todd".to_string(),
        age: 42,
    };

    let p2 = p.clone();
    // let p2 = p;  borrow of moved value

    p.name.push_str(" Pickell");

    println!("p = {:?}, p2 = {:?}", p, p2);

    let mut pnt = Point::new(3, 4);
    let pnt2 = pnt;
    pnt.x += 3;

    println!("pnt = {:?}, pnt2 = {:?}", pnt, pnt2)
}
