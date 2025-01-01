pub fn from_and_into() {
  let my_str = "Hey there";
  let my_string = String::from(my_str);

  println!("my_str: {}", my_str);
  println!("my_string: {}", my_string);
}

use std::convert::From;

#[derive(Debug)]
struct Number {
  #[allow(dead_code)]
  value: i32,
}

impl From<i32> for Number {
  fn from(item: i32) -> Self {
      return Number { value: item };
  }
}

pub fn exercise_from() {
  let number = Number::from(30);
  println!("{:?}", number);
}

use std::convert::Into;

#[derive(Debug)]
struct Float {
  #[allow(dead_code)]
  value: f32,
}

impl Into<Float> for f32 {
  fn into(self) -> Float {
    //return Number::from(self);
    return Float {value: self};
  }
}

pub fn exercise_into() {
  let fl: f32 = 2.4_f32;
  let float: Float = fl.into();
  println!("{:?}", float);
}

use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug,PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
  type Error = ();
  fn try_from(value: i32) -> Result<Self, Self::Error> {
    if value % 2 == 0 {
      Ok(EvenNumber(value))
    }
    else {
      Err(())
    }
  }  
}

pub fn exercise_try_from_into() {
  // TryFrom

  assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
  assert_eq!(EvenNumber::try_from(5), Err(()));

  // TryInfo

  let result: Result<EvenNumber, ()> = 8i32.try_into();
  assert_eq!(result, Ok(EvenNumber(8)));

  let result: Result<EvenNumber, ()> = 3i32.try_into();
  assert_eq!(result, Err(()));
}

use std::fmt;

struct Circle {
  radius: i32
}

impl fmt::Display for Circle {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Circle of radius {}", self.radius)
  }
}

pub fn exercise_display() {
  let circle = Circle { radius: 6 };
  println!("{}", circle.to_string());
}

pub fn exercise_from_str() {
  let parsed: i32 = "5".parse().unwrap();
  let turbo_parsed = "10".parse::<i32>().unwrap();

  let sum = parsed + turbo_parsed;
  println!("Sum: {:?}", sum);

  let not_parsed = "shjidweqejf".parse::<i32>();
  if not_parsed.is_err() {
    println!("can't parse");
  } else {
    println!("{:?}", not_parsed.unwrap());
  }
}
