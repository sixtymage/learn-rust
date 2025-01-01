#[cfg(target_os = "linux")]
fn are_you_on_linux() {
  println!("You are running on linux");
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
  println!("You are *not* running on linux");
}

pub fn main() {
  are_you_on_linux();

  println!("Are you sure?");

  if cfg!(target_os = "linux") {
    println!("Yes, It's definitely linux!");
  } else {
    println!("Yes. It's definitely *not* linux!")
  }
}

#[cfg(conditional = "foo")]
fn get_conditional() -> String {
  return "conditional".to_owned();
}

#[cfg(not(conditional = "foo"))]
fn get_conditional() -> String {
  return "not conditional".to_owned();
}

pub fn main2() {
  println!("Custom: {}", get_conditional());
}
