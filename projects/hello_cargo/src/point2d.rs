use std::fmt;

// a structure to hold two points. Debug and Display implemented for contrast
#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return write!(f, "({}, {})", self.0, self.1);
  }
}

#[derive(Debug)]
struct Point2d {
  x: f64,
  y: f64,
}

impl fmt::Display for Point2d {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return write!(f, "x: {}, y: {}", self.x, self.y);
  }
}

#[derive(Debug)]
struct Complex {
  real: f64,
  imaginary: f64,
}

impl fmt::Display for Complex {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return write!(f, "{} + {}i", self.real, self.imaginary);
  }
}

pub fn display_point2d() {
  let minmax = MinMax(-2, 10);

  println!("Compare foos:");
  println!("Display: {}", minmax);
  println!("Debug: {:?}", minmax);

  let big_range = MinMax(-300, 300);
  let small_range = MinMax(-3, 3);

  println!(
    "The big range is {big} and the small range is {small}",
    big = big_range,
    small = small_range
  );

  let point = Point2d { x: -3.0, y: 4.0 };

  println!("Compare points:");
  println!("Display: {}", point);
  println!("Debug: {:?}", point);

  //println!("The point in binary: {:b}", point);

  let z: Complex = Complex {
    real: 4.0,
    imaginary: 3.0,
  };

  println!("Compare complex:");
  println!("Display: {}", z);
  println!("Debug: {:?}", z);
}
