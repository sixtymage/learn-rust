pub fn main() {
  use std::mem;

  let colour = String::from("green");

  // borrow by ref for the closure
  let print = || println!("colour: {}", colour);
  print();

  // borrow again
  let _reborrow = &colour;
  print();

  // a move is allowed after final use of print
  let _colour_moved = colour;


  let mut count = 0;

  // mut required because borrows &mut stored inside
  let mut inc = || {
    count += 1;
    println!("count: {}", count);
  };

  // call using mutable borrow
  inc();

  // can't reborrow as closure has it
  //let _reborrow = &count;

  // called later
  inc();

  // closure not called after here, possible to reborrow
  let _count_reborrowed = &count;

  let movable = Box::new(3);

  let consume = || {
    println!("movable: {:?}", movable);
    mem::drop(movable);
  };

  consume();

  // will be an error - can consume only once
  //consume();
}

pub fn main2() {
  let haystack = vec![1, 2, 3];

  let contains = move |needle| haystack.contains(needle);

  println!("{}", contains(&1));
  println!("{}", contains(&4));

  // error - can't re-use after it has been moved to the closure
  //println!("{} elements in vec", haystack.len());

  // removing `move` from signature will cause immutable borrow, so .len is still available
}
