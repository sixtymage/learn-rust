pub fn add(left: usize, right: usize) -> usize {
  left + right
}

pub fn subtract(left: i64, right: i64) -> i64 {
  left - right
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }

  #[test]
  fn given_two_numbers_when_subtracted_should_be_correct_answer() {
    let result = subtract(3, 4);
    assert_eq!(result, -1);
  }
}
