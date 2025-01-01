use std::mem;

// borrow a slice
fn analyze_slice(slice: &[i32]) {
  println!("slice[0]: {}", slice[0]);
  println!("slice.length: {}", slice.len());
}

pub fn exercise_array() {
  let xs: [i32; 5] = [1, 2, 3, 4, 5];
  let ys: [i32; 500] = [0; 500];

  // indexing
  println!("xs[0]: {}", xs[0]);
  println!("xs[2]: {}", xs[2]);
  println!("xs.len(): {}", xs.len());

  // stack allocated
  println!("xs occupies {} bytes", mem::size_of_val(&xs));

  // borrow whole
  analyze_slice(&xs);

  // borrow section
  analyze_slice(&ys[2..3]);

  // borrow section
  analyze_slice(&xs[3..4]);

  // empty slice
  let empty_array: [u32;0] = [];
  assert_eq!(&empty_array, &[]);
  assert_eq!(&empty_array, &[][..]);

  // safe access
  for i in 0..xs.len() + 1 {
    match xs.get(i) {
      Some(xval) => println!("xs[{}]: {}", i, xval),
      None => println!("Slow down! {} is too far", i),
    }
  }

  // compile time out of bounds
  //println!("xs[5]: {}", xs[5]);

  // run time out of bounds
  //println!("xs[1..2][3]: {}", xs[1..2][3]);
  println!("xs[1..2][0]]: {}", xs[1..2][0]);

}
