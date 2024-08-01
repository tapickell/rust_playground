use std::fmt::Debug;

extern crate test;

#[path = "../src/b_rand.rs"]
mod b_rand;

pub fn pivot<T: PartialOrd>(v: &mut [T]) -> usize {
  let mut p = b_rand::rand(v.len());
  v.swap(p, 0);
  p = 0;
  for i in 1..v.len() {
    if v[i] < v[p] {
      v.swap(p + 1, i);
      v.swap(p, p + 1);
      p += 1
    }
  }
  p
}

// If already sorted runs at n^2
// If not then can be better
pub fn quick_sort<T: PartialOrd + Debug>(v: &mut [T]) {
  if v.len() <= 1 {
    return;
  }
  let p = pivot(v);
  println!("{:?}", v);

  let (a, b) = v.split_at_mut(p);
  quick_sort(a);
  quick_sort(&mut b[1..]);
}

struct RawSend<T>(*mut [T]);

unsafe impl<T> Send for RawSend<T> {}

// uses 2^log(n) threads
pub fn quick_sort_threaded<T: 'static + PartialOrd + Debug + Send>(v: &mut [T]) {
  if v.len() <= 1 {
    return;
  }
  let p = pivot(v);
  println!("{:?}", v);

  let (a, b) = v.split_at_mut(p);
  let raw_a: *mut [T] = a as *mut [T];
  let raw_s = RawSend(raw_a);
  unsafe {
    let handle = std::thread::spawn(move || {
      quick_sort_threaded(&mut *raw_s.0);
    });
    quick_sort_threaded(&mut b[1..]);
    handle.join().ok();
  }
}

#[cfg(test)]
mod benches {
  use super::*;
  use test::{black_box, Bencher};

  #[bench]
  fn bench_quick_sort_threaded(b: &mut Bencher) {
    b.iter(|| {
      let mut v1 = vec![4, 6, 1, 8, 11, 13, 3, 5, 11, 56, 8, 1];
      quick_sort_threaded(&mut v1);
    });
  }
}
