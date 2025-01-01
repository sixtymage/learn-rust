
pub fn main() {
  let outer_var = 42;

  // no access - error
  //fn some_function(i: i32) -> i32 { return i + outer_var }

  let closure_annotated = |i: i32| -> i32 {i + outer_var};
  let closure_inferred = |i| i + outer_var;

  // call closures
  println!("annotated: {}", closure_annotated(3));
  println!("inferred: {}", closure_inferred(4));

  // error - can't use with other type
  //println!("inferred with float {}", closure_inferred(3.2));

  // return type inference
  let one = || 1;
  println!("one: {}", one());
}
