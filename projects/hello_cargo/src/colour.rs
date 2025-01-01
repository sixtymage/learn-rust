use std::fmt::{self, Formatter, Display};

#[derive(Debug)]
struct Colour {
  red: u8,
  green: u8,
  blue: u8,
}

impl Display for Colour {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    return write!(f, "RGB ({}, {}, {}) 0x{0:0>2X}{1:0>2X}{2:0>2X}", self.red, self.green, self.blue);
  }
}

pub fn display_colour() {
  for colour in [
    Colour {red: 128, green: 255, blue: 90},
    Colour {red: 0, green: 3, blue: 254},
    Colour {red: 0, green: 0, blue: 0},
  ] {
    println!("{}", colour);
  }
}
