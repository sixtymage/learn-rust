
fn apply<F>(f: F) where 
  F: FnOnce() {
  f();
}

pub fn main() {
  let x = 8;

  let print = || println!("{}", x);

  apply(print);
}

