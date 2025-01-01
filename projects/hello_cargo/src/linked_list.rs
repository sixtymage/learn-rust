use crate::linked_list::List::*;

enum List {
  // element that wraps item and pointer to next
  Cons(u32, Box<List>),
  // element that marks the end of the list
  Nil,
}

// attach methods to an enum
impl List {
  fn new() -> List {
    return Nil;
  }

  // add new element to front of provided list
  fn prepend(self, elem: u32) -> List {
    return Cons(elem, Box::new(self));
  }

  // length of list
  fn len(&self) -> u32 {
    match *self {
      Cons(_, ref tail) => 1 + tail.len(),
      Nil => 0
    }
  }

  fn stringify(&self) -> String {
    match *self {
      Cons(head, ref tail) => {
        return format!("{}, {}", head, tail.stringify());
      },
      Nil => {
        return format!("Nil");
      },
    }
  }
}

pub fn exercise_list() {
  let mut list = List::new();

  list = list.prepend(1);
  list = list.prepend(2);
  list = list.prepend(4);

  let l = list.len();
  println!("len = {}", l);
  println!("list = {}", list.stringify());
}
