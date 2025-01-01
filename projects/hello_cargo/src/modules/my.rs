pub mod nested;
mod private_nested;

fn private_function() {
  println!("called `my::private_function`");
}

pub fn function() {
  println!("called `my::public_function`");
}

pub fn indirect_access() {
  print!("called `my::indirect_access` that ");
  private_function();
}

pub fn call_public_function_in_my_mod() {
  print!("called `my::call_public_function_in_my_mod` that ");
  nested::public_function_in_my_mod();
  print!("> ");
  nested::public_function_in_super_mod();
}

pub(crate) fn public_function_in_crate() {
  println!("called `my::public_function_in_crate`")
}

pub struct OpenBox<T> {
  pub contents: T,
}

pub struct ClosedBox<T> {
  #[allow(dead_code)]
  contents: T,
}

impl<T> ClosedBox<T> {
  pub fn new(contents: T) -> ClosedBox<T> {
    ClosedBox {
      contents: contents,
    }
  }
}

mod cool {
  pub fn function() {
    println!("called `my::cool::function`");
  }
}

pub fn indirect_call() {
  println!("called `my::indirect_call`, that\n> ");

  // `self` == current module scope
  self::function();
  function();

  // access another module inside my2
  self::cool::function();

  // parent scope
  super::function();

  // bind cool::function to the *crate* scope
  {
    use crate::modules::cool::function as root_function;
    root_function();
  }
}
