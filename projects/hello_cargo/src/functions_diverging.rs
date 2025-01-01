pub fn main() {
  fn sum_odd_numbers(up_to: u32) -> u32 {
    let mut acc = 0;

    for i in 0..up_to {
      // addition demands a u32 be on the rhs
      // the match on false doesn't return an integer but rather the "never" return ! 
      // so it doesn't conflict with the type requirements of the match expression
      let addition: u32 = match i%2 == 1 {
        true => i,
        false => continue,
      };
      acc += addition;
    }
    acc 
  }

  println!("sum of odd numbers up to 9 (excl.): {}", sum_odd_numbers(9));
}
