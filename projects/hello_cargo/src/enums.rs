#![allow(dead_code)]

enum WebEvent {
  // unit-like
  PageLoad,
  PageUnload,
  // like a tuple-struct
  KeyPress(char),
  Paste(String),
  // like a c-struct
  Click{ x: u64, y: u64}
}

fn inspect(event: WebEvent) {
  match event {
    WebEvent::PageLoad => println!("Page Loaded!"),
    WebEvent::PageUnload => println!("Page Unloaded!"),
    // destruct the char
    WebEvent::KeyPress(c) => println!("Pressed '{}'", c),
    WebEvent::Paste(s) => println!("Pasted \"{}\"", s),
    // destruct click coords
    WebEvent::Click{x: xpos, y: ypos} => println!("Clicked ({}, {})", xpos, ypos),
  }
}

// alias
type WebEv = WebEvent;

impl WebEvent {
  fn load_or_unload(&self) {
    match self {
      Self::PageLoad => println!("PageLoad"),
      Self::PageUnload => println!("PageUnLoad"),
      _ => println!("Nothinbg"),
    }
  }
}

pub fn exercise_enum() {
  let pressed: WebEvent = WebEvent::KeyPress('x');
  inspect(pressed);

  let pasted: WebEvent = WebEvent::Paste("my text".to_owned());
  inspect(pasted);

  let click: WebEvent = WebEvent::Click { x: 123, y: 345 };
  inspect(click);

  let load: WebEvent = WebEvent::PageLoad;
  inspect(load);

  let unload: WebEvent = WebEvent::PageUnload;
  inspect(unload);

  let abbr: WebEv = WebEv::PageLoad;
  inspect(abbr);

  let test: WebEvent = WebEvent::PageLoad;
  WebEv::load_or_unload(&test);
}

enum Status {
  Rich,
  Poor,
}

enum Work {
  Civilian,
  Soldier,
}

pub fn exercise_use_enums() {
  // explicit use
  use crate::enums::Status::{Poor, Rich};

  // use all
  use crate::enums::Work::*;

  let status = Poor;
  let work = Civilian;

  match status {
    Rich => println!("Lots of money"),
    Poor => println!("No money"),
  }

  match work {
    Soldier => println!("Semper fi"),
    Civilian => println!("Numquam fi"),
  }
}

enum Number {
  Zero,
  One,
  Two,
}

enum Colour {
  Red = 0xff0000,
  Green = 0x00ff00,
  Blue = 0x0000ff,
}

pub fn exercise_cstyle_enums() {

  // cast as integers
  println!("zero is {}", Number::Zero as i32);
  println!("one is {}", Number::One as i32);

  println!("roses are #{:06x}", Colour::Red as u32);
  println!("violets are #{:06x}", Colour::Blue as u32);
  println!("all my base are belong to you");
}
