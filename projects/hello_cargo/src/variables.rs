pub fn exercise_bindings() {
  let an_integer = 1u32;
  let a_boolean = true;
  let unit = ();

  let copied_integer  = an_integer;

  println!("A (copied) integer {:?}", copied_integer);
  println!("A boolean {:?}", a_boolean);
  println!("Meet the unit value: {:?}", unit);

  let _unused = 3u32;
  //let noisy_unused = 4u32;
}

pub fn exercise_mutability() {
  let _immutable_binding = 1;
  let mut mutable_binding = 2;

  println!("before mutation: {}", mutable_binding);

  mutable_binding += 2;
  println!("after mutation: {}", mutable_binding);

  // error!
  //_immutable_binding += 1;
}

pub fn exercise_scope() {
  let long_lived_binding = 1;

  {
    let short_lived_binding = 2;
    println!("inner short: {}", short_lived_binding);
  }

  //println!("outer short: {}", short_lived_binding);

  println!("outer long: {}", long_lived_binding);
}

pub fn exercise_shadowed() {
  let shadowed_binding = 1;
  println!("initial: {}", shadowed_binding);

  {
    let shadowed_binding = 2;
    println!("inner: {}", shadowed_binding);
  }

  println!("pre-final: {}", shadowed_binding);

  let shadowed_binding = 4;
  println!("final: {}", shadowed_binding);
}

pub fn declare_first() {
  let a_binding;

  {
    let x = 2;
    a_binding = x * x;   
  }

  println!("a_binding: {}", a_binding);

  let another_binding;

  // error
  //println!("another_binding: {}", another_binding);

  another_binding = 1;
  println!("another_binding: {}", another_binding);
}

pub fn freezing() {
  let mut _mutable_integer = 7i32;

  {
    let _mutable_integer = _mutable_integer;
    //_mutable_integer = 50;
  }

  _mutable_integer = 3;
}