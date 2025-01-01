pub fn function() {
  println!("called `my::nested::function`");
}

#[allow(dead_code)]
fn private_function() {
  println!("called `my::nested::private_function`");
}

pub(in crate::modules::my) fn public_function_in_my_mod() {
  print!("called `my::nested::public_function_in_my_mod` that ");
  public_function_in_nested();
}

pub(self) fn public_function_in_nested() {
  println!("called `my::nested::public_function_in_nested`");
}

pub(super) fn public_function_in_super_mod() {
  println!("called `my::public_function_in_super_mod`");
}
