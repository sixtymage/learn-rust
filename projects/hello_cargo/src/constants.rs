// globals
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
  n > THRESHOLD
}

pub fn exercise_constants() {
  let n = 16;

  println!("This is {}", LANGUAGE);
  println!("The threshold is {}", THRESHOLD);
  println!("{} is {}", n, if is_big(n) { "big" } else { "small" } );

  // cannot do this
  //THRESHOLD = 3;
}