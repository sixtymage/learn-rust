pub fn formatted_print() {
  println!("{} days", 31);

  println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");

  println!(
    "Hello {customer}, I am {assistant}, how may I help?",
    customer = "Jerry",
    assistant = "Sue"
  );

  const NUMBER: i32 = 31;
  println!("Base 10: {}", NUMBER);
  println!("Base 2: {:b}", NUMBER);
  println!("Base 16: {:X}", NUMBER);

  println!("{number:0>width$}", number = NUMBER, width = 5);

  #[allow(dead_code)]
  struct Foo(i32);
  //println!("This won't work {}", Foo(3));

  let number: f64 = 1.238;
  let width: usize = 5;
  println!("{number:0>width$}");
}

#[derive(Debug)]
struct Foo(i32);

#[derive(Debug)]
struct Deep(Foo);

pub fn debug_print() {
  println!("{:?} months in a year", 12);
  println!(
    "{1:?} {0:?} is the {actor:?} name",
    "Slater",
    "Christian",
    actor = "actor's"
  );

  println!("Now {:?} will print!", Foo(3));
  println!("Now {:?} will print!", Deep(Foo(7)));

  pretty_print();
}

#[derive(Debug)]
struct Person<'a> {
  #[allow(dead_code)]
  name: &'a str,
  #[allow(dead_code)]
  age: u8,
}

fn pretty_print() {
  let name: &str = "Peter";
  let age: u8 = 27;
  let peter = Person { name, age };

  println!("{:#?}", peter);
}

pub fn custom_display() {
  println!("{}", Foo(24));
}

use std::fmt;

impl fmt::Display for Foo {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return write!(f, "{}", self.0);
  }
}
