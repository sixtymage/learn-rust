pub fn exercise_expressions() {
  let x = 5u32;

  let y = {
    let x_squared = x * x;
    let x_cubed = x_squared * x;

    // assigned to y
    x_cubed + x_squared + x
  };

  let z = {
    // semicolon suppresses the expression and () is assigned to z
    let _ = 2 * x;
  };

  println!("x: {:?}", x);
  println!("y: {:?}", y);
  println!("z: {:?}", z);
}