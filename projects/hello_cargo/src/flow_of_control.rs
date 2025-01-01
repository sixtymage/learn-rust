pub fn if_else() {
  let n = 2001;

  if n < 0 {
    print!("{} is negative", n);
  } else if n > 0 {
    print!("{} is positive", n);
  } else {
    print!("{} is zero", n);
  }

  let big_n = 
    if n > -10 && n < 10 {
      println!(" and is a small number, increase ten-fold");
      n * 10
    } else {
      println!(" and is a large number, halve it");

      n / 2
    };
  
  println!("n: {}, big_n: {}", n, big_n);
}

pub fn exercise_loop() {
  let mut count = 0u32;

  loop {
    count += 1;

    if count == 3 {
      println!("three!");

      // skip
      continue;
    }

    println!("{}", count);

    if count  == 5 {
      println!("five - enough!");

      break;
    }
  }
}

#[allow(unreachable_code)]
pub fn exercise_inner_break() {
  'outer: loop {
    println!("entered outer loop");

    loop {
      println!("entered inner loop");

      // break only the inner loop
      //break;

      // break outer
      break 'outer;

    }

    println!("end of outer loop");
  }

  println!("exited outer loop");
}

pub fn exercise_loop_return() {
  let mut count = 0;

  let a = 
    loop {
      count += 1;

      if count == 10 {
        break count * 2;
      }
    };

  println!("a: {}", a);
}

fn fizzbuzz(number: i32) {
  if number % 15 == 0 {
    println!("fizzbuzz");
  } else if number % 3 == 0 {
    println!("fizz");
  } else if number % 5 == 0 {
    println!("buzz");
  } else {
    println!("{}", number);
  }
}

pub fn exercise_while() {
  let mut number = 1;

  while number < 101 {
    fizzbuzz(number);
    number += 1;
  }
}

pub fn for_and_range1() {
  for number in 1..101 {
    fizzbuzz(number);
  }
}

pub fn for_and_range2() {
  for number in 1..=100 {
    fizzbuzz(number);
  }
}

pub fn for_with_iter() {
  let names = vec!["Bob", "Frank", "Ferris"];

  for name in names.iter() {
    let _foo: &&str = name;

    match name {
      &"Ferris" => println!("There is a rustacean amongst us!"),
      _ => println!("Hello, {}", name),
    }
  }

  println!("names: {:?}", names);
}

pub fn for_with_into_iter() {
  let names = vec!["Bob", "Frank", "Ferris"];

  for name in names.into_iter() {
    let _foo: &str = name;

    match name {
      "Ferris" => println!("There is a rustacean amongst us!"),
      _ => println!("Hello, {}", name),
    }
  }

  // can't touch the array as the iterator has claimed the entries
  //println!("names: {:?}", names);
}

pub fn for_with_iter_mut() {
  let mut names = vec!["Bob", "Frank", "Ferris"];

  for name in names.iter_mut() {
    let _foo: &mut &str = name;

    *name = match name {
      &mut "Ferris" => "There is a rustacean amongst us!",
      _ => "Hello",
    }
  }

  println!("names: {:?}", names);
}

pub fn exercise_match1() {
  let number = 21;

  println!("Tell me about {}", number);
  match number {
    1 => println!("One!"),
    // many
    2 | 3 | 5 | 7 | 11 => println!("Prime"),
    // range
    13..=19 => println!("Teen"),
    _ => println!("Aint special"),
  }
}

pub fn exercise_match2() {
  let boolean = true;

  let binary = match boolean {
    false => 0,
    true => 1,
  };

  println!("{} -> {}", boolean, binary);
}

pub fn destructure_match() {
  let triple = (3, -2, 4);

  println!("tell me about {:?}", triple);

  match triple {
    (0, y, z) => println!("first: 0, second: {:?}, third: {:?}", y, z),
    (1, ..) => println!("first is 1, rest irrelevant"),
    (..,2) => println!("last is 2, rest irrelevant"),
    (3, .., 4) => println!("first is 3, last is 4, rest irrelevant"),
    _ => println!("all irrelevant"),
  }
}

pub fn destructure_array_slice() {
  let array = [12, 2, 3, 4, -6];

  println!("tell me about {:?}", array);

  match array {
    // [0, second, third] => println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),
    // [1, _, third] => println!("array[0] = 1, ignore, array[2] = {}", third),
    [-1, second, ..] => println!("array[0] = -1, array[1] = {}, ignore", second),
    // won't compile
    //[-1, second] => println!("something bad"),
    [3, second, tail @ ..] => println!("array[0] = 3, array[1] = {}, rest: {:?}", second, tail),
    [first, middle @ .., last] => println!("first = {}, middle = {:?}, last = {}", first, middle, last),
  }
}

#[allow(dead_code)]
enum Colour {
  Red,
  Green,
  Blue,
  RGB(u32,u32,u32),
  HSV(u32,u32,u32),
  HSL(u32,u32,u32),
  CMY(u32,u32,u32),
  CMYK(u32,u32,u32,u32),
}

pub fn destructure_enum() {
  //let colour = Colour::RGB(122, 17, 40);
  //let colour = Colour::HSV(12, 13, 145);
  let colour = Colour::CMYK(13, 87, 231, 21);

  println!("What colour is it?");

  match colour {
    Colour::Red => println!("Red"),
    Colour::Green => println!("Green"),
    Colour::Blue => println!("Blue"),
    Colour::RGB(r, g, b) => println!("red {}, green {}, blue {}", r, g, b),
    Colour::HSV(h, s, v) => println!("hue {}, saturation {}, value {}", h, s, v),
    Colour::HSL(h, s, l) => println!("hue {}, saturation {}, lightness {}", h, s, l),
    Colour::CMY(c, m, y) => println!("cyan {}, magenta {}, yellow {}", c, m, y),
    Colour::CMYK(c, m, y, k) => println!("cyan {}, magenta {}, yellow {}, key (black) {}!", c, m, y, k),
  }
}

pub fn destructure_pointers() {
  let reference = &4;

  // match using reference
  match reference {
    &val => println!("got value by destructure: {}", val),
  }

  // dereference then match
  match *reference {
    val => println!("got value by dereferencing: {}", val),
  }

  // define reference explicitly
  let _not_a_reference = 3;
  let ref _is_a_reference = 3;

  // get reference from value or mutable value
  let value = 5;
  let mut mut_value = 6;

  match value {
    ref r => println!("got a reference to a value {:?}", r),
  }

  match mut_value {
    ref mut mr => {
      println!("got a reference to a mutable value m = {}, mr = {:?}", *mr, mr);
      *mr = *mr + 1;
      println!("updated mutable value m = {}, mr = {:?}", *mr, mr);
    }
  }
}

pub fn destructure_struct() {

  struct Foo {
    x: (u32, u32),
    y: u32,
  }

  let foo = Foo {x: (4, 2), y: 3};

  match foo {
    Foo {x: (1, b), y} => println!("First of x is 1, b = {}, y = {}", b, y),
    Foo {y: 2, x: i} => println!("x is {:?}, y is 2", i),
    Foo {y, ..} => println!("y is {}, rest ignored", y),
    //Foo {y} => println!("an error"),
  }

  let faa = Foo {x: (1,2), y: 3};

  let Foo{x:j, y: z} = faa;
  println!("x is {:?}, y is {}", j, z);
}

#[allow(dead_code)]
enum Temperature {
  Celsius(i32),
  Fahrenheit(i32),
}

pub fn match_guards() {
  //let temperature = Temperature::Celsius(25);
  let temperature = Temperature::Fahrenheit(87);

  match temperature {
    Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
    Temperature::Celsius(t) => println!("{}C is not above 30 Celsius", t),
    Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
    Temperature::Fahrenheit(t) => println!("{}F is not above 86 Fahrenheit", t),
  }
}

pub fn match_guards2() {
  let number: u8 = 4;

  match number {
    i if i == 0 => println!("Zero!"),
    i if i > 0 => println!("> 0"),
    _ => unreachable!("should never happen"),
  }
}

fn age() -> u32 {
  22
}

pub fn match_binding() {
  println!("Tell me what kind lahnie");

  match age() {
    0 => println!("not first birthday yet"),
    n @ 1..=12 => println!("child of age {}", n),
    n @ 13..=19 => println!("teenager {}", n),
    n => println!("grumpy old person of age {}", n),
  }
}

fn some_number() -> Option<u32> {
  let t = std::time::SystemTime::now();
  let d = t.duration_since(std::time::UNIX_EPOCH).expect("all good");
  let m = d.as_millis();

  if m % 2 == 0 {
    let n: u32 = ((m % 5) as u32) + 40;
    return Some(n);
  } else {
    return None;
  }
}

pub fn match_binding2() {
  match some_number() {
    Some(n @ 42) => println!("The Answer: {}!", n),
    Some(n) => println!("Less than great: {}", n),
    _ => println!("none"),
  }
}

pub fn if_let() {

  // somewhat messy approach:

  let optional = Some(7);

  match optional {
    Some(i) => {
      println!("A really long string and `{:?}`", i);
    },
    _ => {},
  }

  // less clumsy approach:
  let number = Some(7);
  let letter: Option<i32> = None;
  let emoticon: Option<i32> = None;

  // reads: if `let` destructures `number` into `Some(i)`, evaluate the block
  if let Some(i) = number {
    println!("Matched {:?}!", i);
  }

  if let Some(i) = letter {
    println!("Matched {:?}!", i);
  } else {
    println!("Match failed");
  }

  let i_like_letters = false;

  if let Some(i) = emoticon {
    println!("Matched {:?}!", i);
  } else if i_like_letters {
    println!("Didn't match number, going with letter");
  } else {
    println!("Go with emoticon");
  }

}

enum Foo {
  Bar,
  Baz,
  Qux(u32),
}

pub fn if_let2() {
  let a = Foo::Bar;
  let b = Foo::Baz;
  let c = Foo::Qux(100);

  if let Foo::Bar = a {
    println!("a is foobar");
  }

  if let Foo::Bar = b {
    println!("b is foobar");
  }

  if let Foo::Qux(value) = c {
    println!("c is qux({})", value);
  }

  if let Foo::Qux(v @ 100) = c {
    println!("c is qux({:?})", v);
  }
}

enum Foo2 {
  Bar2,
  #[allow(dead_code)]
  Baz2,
}

pub fn if_let3() {
  let a = Foo2::Bar2;

  if let Foo2::Bar2 = a {
    println!("a is foobar");
  }
}

use std::str::FromStr;

fn get_count_item(s: &str) -> (u64, &str) {

  let mut it = s.split(' ');
  let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
    panic!("Can't segment count item pair: '{s}'");
  };

  let Ok(count) = u64::from_str(count_str) else {
    panic!("Can't parse integer: '{s}'");
  };

  (count, item)
}

fn get_count_item_worse(s: &str) -> (u64, &str) {
  let mut it = s.split(' ');

  let (count_str, item) = match (it.next(), it.next()) {
    (Some(count_str2), Some(item2)) => (count_str2, item2),
    _ => panic!("Can't get segments"),
  };

  let count = if let Ok(count2) = u64::from_str(count_str) {
    count2
  } else {
    panic!("Can't parse integer");
  };

  (count, item)
}

pub fn let_else() {
  assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
  assert_eq!(get_count_item_worse("4 chairs"), (4, "chairs"));
}

fn while_let_worse() {
  let mut optional = Some(0);

  loop {
    match optional {
      Some(i) => {
        if i > 9 {
          println!(">9, quit");
          optional = None;
        } else {
          println!("`i` is `{:?}`. Try again.", i);
          optional = Some(i+1);
        }
      },
      _ => {
        break;
      }
    }
  }
}

fn while_let_better() {
  let mut optional = Some(0);

  while let Some(i) = optional {
    if i > 9 {
      println!(">9, quit");
      optional = None;
    } else {
      println!("`i` is `{:?}`. Try again.", i);
      optional = Some(i+1);
    }
  }
}

pub fn while_let() {
  while_let_worse();
  while_let_better();
}
