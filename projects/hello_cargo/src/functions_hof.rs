fn is_odd(n: u32) -> bool {
  n % 2 == 1
}

pub fn main() {

  println!("sum of odd squares < 1000");

  let upper = 1000;
  
  // imperative approach
  let mut acc = 0;
  for n in 0.. {
    let n_squared = n * n;

    if n_squared >= upper {
      break;
    }
    
    if is_odd(n_squared) {
      acc += n_squared;
    }
  }
  println!("imperative style: {}", acc);

  // declarative style
  let sum_of_squared_odd_numbers: u32 = 
    (0..)
    .map(|n| n*n)
    .take_while(|&n_squared| n_squared < upper)
    .filter(|&n_squared| is_odd(n_squared))
    .sum();
  println!("functional style: {}", sum_of_squared_odd_numbers);

}
