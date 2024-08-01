pub struct Stepper {
    curr: i32,
    step: i32,
    max: i32,
}

impl Iterator for Stepper {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.curr > self.max {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}

fn main() {
    let mut n = 0;
    while n < 10 {
        n += 1;
        println!("Hello, world! {}", n);
    }

    for i in 0..10 {
        println!("Hello, world! for in {}", i);
    }

    let mut st = Stepper {
        curr: 2,
        step: 3,
        max: 15,
    };
    loop {
        match st.next() {
            Some(v) => println!("loop value {}", v),
            None => break,
        }
    }

    let mut st2 = Stepper {
        curr: 2,
        step: 4,
        max: 15,
    };
    while let Some(n) = st2.next() {
        println!("while value {}", n);
    }

    let st3 = Stepper {
        curr: 5,
        step: 5,
        max: 50,
    };

    for i in st3 {
        println!("for value {}", i)
    }
}
