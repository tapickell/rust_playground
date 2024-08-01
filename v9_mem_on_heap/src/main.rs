#[derive(Debug)]
pub struct LinkedList<T> {
    data: T,
    next: Option<Box<LinkedList<T>>>,
}

impl<T: std::ops::AddAssign> LinkedList<T> {
    pub fn add_up(&mut self, n: T) {
        self.data += n;
    }
}

fn main() {
    let mut ll = LinkedList {
        data: 3,
        next: Some(Box::new(LinkedList {
            data: 2,
            next: None,
        })),
    };

    // if /let Some(...)/ pattern matches ll.next
    if let Some(ref mut v) = ll.next {
        v.add_up(10)
    }

    let mut v: Vec<String> = Vec::new();
    v.push("heya".to_string());
    v.push("wassup".to_string());

    println!("v.len = {}, v.capacity = {}", v.len(), v.capacity());

    println!("Hello, world!, {:?}", ll);

    let s = " hello "; // pointer to memory that does not change
    let p = s.trim(); // substring of that memory slice not a vec
    println!("p == |{}|, s == |{}|", p, s);

    let mut st = "Hello ".to_string(); // on heap like a vec
    let pt = st.trim();
    st.push_str("World");
    st.push('!');

    let sparkle_heart = vec![240, 159, 146, 150];
    let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();

    println!("Awwww {}", sparkle_heart);

    let fstr = "help me find home";
    println!("ffstr = {}", string_stuff(fstr));
    println!("choosen = {}", choose_str(1));
}

// String is a str in a Box
// Vec is an Atray in a Box
fn string_stuff(s: &str) -> &str {
    for (n, x) in s.char_indices() {
        if x == 'f' {
            return &s[n..];
        }
    }
    s
}

fn choose_str(n: i32) -> &'static str {
    match n {
        0 => "hello",
        1 => "goodbye",
        _ => "other",
    }
}
