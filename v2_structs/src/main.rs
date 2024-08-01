#[derive(Debug)]
pub struct Person {
    name: String,
    age: i32,
    children: i32,
    fave_color: Color,
}

#[derive(Debug)]
pub enum Color {
    Purple(String),
    Red,
    Green,
    Blue,
}

impl Person {
    pub fn print(self) -> String {
        format!(
            "name= {}, age= {}, children {}",
            self.name, self.age, self.children
        )
    }
}

fn main() {
    let p = Person {
        name: "Todd".to_string(),
        age: 42,
        children: 1,
        fave_color: Color::Purple("Grimace".to_string()),
    };

    let c = Color::Purple("Prince".to_string());

    match c {
        Color::Purple(s) => println!("It is {}", s),
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
    }

    println!("Hello, world from {:?}", p);
}
