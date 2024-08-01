use std::sync::Mutex;

lazy_static::lazy_static! {
  static ref RG: Mutex<RandGen> = Mutex::new(RandGen::new(34052));
}

pub fn rand(max: usize) -> usize {
  RG.lock().unwrap().next_value(max)
}

pub struct RandGen {
  curr: usize,
  mul: usize,
  inc: usize,
  modulo: usize,
}

impl RandGen {
  pub fn new(curr: usize) -> Self {
    RandGen {
      curr,
      mul: 56394237,
      inc: 34642349,
      modulo: 2325454456,
    }
  }

  pub fn next_value(&mut self, max: usize) -> usize {
    self.curr = (self.curr * self.mul + self.inc) % self.modulo;
    self.curr % max
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_rand_printout() {
    let mut r = RandGen::new(12);
    for _ in 0..100 {
      println!("--{}", r.next_value(100));
    }
  }
}
