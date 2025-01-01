#![allow(overflowing_literals)]

pub fn casting() {
  let decimal = 65.4321f32;

  // error
  //let integer: u8 = decimal;

  // explicit
  let integer = decimal as u8;
  let character = integer as char;

  // error
  //let character2 = decimal as char;

  println!("casting {} -> {} -> {}", decimal, integer, character);

  // casting to unsigned T, T::MAX + 1 is added or subtracted to fit

  println!("1000 as a u16 is: {}", 1000 as u16);

  // 1000 - 256 - 256 - 256 - 232
  // under the hood, the first 8 least significant bits are kept
  // while the rest towards the most significant bit are truncated
  println!("1000 as a u8 is: {}", 1000 as u8);
  // -1 + 256 = 255
  println!("-1 as a u8 is: {} ", (-1i8) as u8);

  // for positive, this is same as mod
  println!("1000 mod 256 is: {}", 1000 % 256);

  // signed values
  println!("128 as i16 is: {}", 128 as i16);
  println!("128 as i8 is: {}", 128 as i8);
  println!("128 as i8 as hex is: 0x{:x}", 128 as i8);
  println!("1000 as u8 is: {}", 1000 as u8);
  println!("1000 as i8 is: {}", 1000 as i8);
  println!("232 as i8 is: {}", 232 as i8);
  println!("1000 as i8 as hex is: 0x{:x}", 1000 as i8);
  println!("232 as i8 as hex is: 0x{:x}", 232 as i8);
  println!("232 as i8 as binary is: 0x{:0b}", 232 as i8);
  println!("129 as i8 is: {}", 129 as i8);
  println!("129 as i8 as hex is: 0x{:x}", 129 as i8);
  println!("129 as i8 as binary is: 0x{:0b}", 129 as i8);

  // floats and ints
  println!("300.0 as u8 is: {}", 300.0_f32 as u8);
  println!("-100.0 as u8 is: {}", -100.0_f32 as u8);
  println!("nan as u8 is: {}", f32::NAN as u8);

  // slight runtime overhead of the above, avoid with
  unsafe {
    println!("unsafe equivalents:");
    println!("300.0 as u8 is: {}", 300.0_f32.to_int_unchecked::<u8>());
    println!("-100.0 as u8 is: {}", (-100.0_f32).to_int_unchecked::<u8>());
    println!("nan as u8 is: {}", f32::NAN.to_int_unchecked::<u8>());
  }
}

pub fn literals() {
  // suffixed
  let x = 1u8;
  let y = 2u32;
  let z = 3f32;

  // unsuffixed
  let i = 1;
  let f = 1.0;

  // size in bytes
  println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
  println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
  println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
  println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
  println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}

pub fn inference() {
  let elem = 5u8;
  let mut vec = Vec::new();

  vec.push(elem);

  println!("{:?}", vec);
}

type Nanosecond = u64;
type Inch = u64;
type U64 = u64;

pub fn aliasing() {

  let nanoseconds: Nanosecond = 5 as U64;
  let inches: Inch = 13 as U64;

  println!("{} ns + {} in = {} unit?", nanoseconds, inches, nanoseconds + inches);
}
