fn call_me<F: Fn()>(f: F) {
  f();
}

fn function() {
  println!("I am a function!");
}

pub fn main() {

  let closure = || println!("A closure!");

  call_me(closure);
  call_me(function);
}
