fn apply<F>(f: F) where 
  F: FnOnce() {

    f();
}

fn apply_to_3<F>(f: F) -> i32 where
  F: Fn(i32) -> i32 {
  
  f(3)
}

pub fn main () {
  use std::mem;

  let greeting = "hello";

  // a non-copy type
  let mut farewell = "goodbye".to_owned();

  // capture greeting by ref, and farewell by value

  let diary = || {

    // by ref, needs Fn
    println!("I said {}", greeting);

    farewell.push_str("!!!");
    println!("Then I screamed {}", farewell);
    println!("Now I sleep.");

    mem::drop(farewell);
  };

  apply(diary);

  let double = |x| 2 * x;
  println!("3 doubled = {}", apply_to_3(double));
}
