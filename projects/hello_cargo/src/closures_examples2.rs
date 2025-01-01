pub fn main() {
  let vec1 = vec![1, 2, 3];
  let vec2 = vec![4, 5, 6];

  let mut iter = vec1.iter();
  let mut into_iter = vec2.into_iter();
  
  println!("find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
  println!("find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

  let array1 = [1, 2, 3];
  let array2 = [4, 5, 6];

  println!("find 2 in a1: {:?}", array1.iter().find(|x| **x == 2));
  println!("find 2 in a2: {:?}", array2.into_iter().find(|x| *x == 2));

  let vec = vec![1, 9, 3, 12, 4, 13, 2];
  let index_of_first_even_number = vec.iter().position(|&x| x % 2 == 0);
  println!("index_of_first_even_number: {:?}", index_of_first_even_number);

  let index_of_first_neg_number = vec.into_iter().position(|x| x < 0);
  assert_eq!(index_of_first_neg_number, None);
}
