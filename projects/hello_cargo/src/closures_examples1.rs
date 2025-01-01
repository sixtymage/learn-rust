pub fn main() {
  let vec1 = vec![1, 2, 3];
  let vec2 = vec![4, 5, 6];

  println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
  println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

  println!("vec1 rides again - len {}", vec1.len());
  // error - vec given to the iterator
  //println!("vec2 len: {}", vec2.len());

  let array1 = [1, 2, 3];
  let array2 = [4, 5, 6];

  println!("2 in a1: {}", array1.iter().any(|&x| x == 2));
  println!("2 in a2: {}", array2.into_iter().any(|x| x == 2));

}