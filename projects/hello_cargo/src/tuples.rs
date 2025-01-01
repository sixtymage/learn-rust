use std::fmt;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
  let (int_param, bool_param) = pair;

  return (bool_param, int_param);
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "( {}, {} )", self.0, self.1)?;
    return write!(f, "( {}, {} )", self.2, self.3);
  }
}

fn transpose(m: Matrix) -> Matrix {
  return Matrix(m.0, m.2, m.1, m.3);
}

pub fn exercise_tuples() {
  // a tuple with a bunch of types
  let long_tuple = (
    1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
  );

  // use indexes
  println!("long_tuple[0]: {}", long_tuple.0);
  println!("long_tuple[1]: {}", long_tuple.1);

  // tuples of tuples
  let tuple_of_tuples = ((1u32, false), (3u16, 2i16, true), 'a');
  println!("tuple_of_tuples: {:?}", tuple_of_tuples);

  // too long tuple
  let _really_long_tuple = (1,2,3,4,5,6,7,8,9,10,11,12,13);
  //println!("really_long_tuple: {:?}", really_long_tuple);

  let pair = (1, true);
  println!("pair is {:?}", pair);

  let reverse_pair = reverse(pair);
  println!("reverse_pair is {:?}", reverse_pair);

  // one element
  let one_element = (1u32,);
  println!("one_element: {:?}", one_element);

  // tuples create bindings
  let tuple = (1, "hello", 4.5, true);
  let (a, b, c, d) = tuple;

  println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

  // matrix
  let m = Matrix(1.2, 1.2, 2.1, 2.2);
  println!("{:?}", m);
  println!("Matrix:\n{}", m);
  println!("Transpose:\n{}", transpose(m));
}
