#[allow(dead_code)]
pub fn function() {
  println!("called private_nested::function");
}

#[allow(dead_code)]
pub(crate) fn restricted_function() {
  println!("called `private_nested::fn_restricted_function`");
}
