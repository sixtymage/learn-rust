mod my;

fn function() {
  println!("called `modules::function`")
}

pub fn main () {
  // avoid name collision
  function();
  my::function();

  // public access
  my::indirect_access();
  my::nested::function();
  my::call_public_function_in_my_mod();
  my::public_function_in_crate();

  // error - only visible in mod
  //my::nested::public_function_in_my_mod();

  // error - can't access private items (even if in public module)
  // my::private_function();
  // my::nested::private_function();
  // my::private_nested::function();
  // my::private_nested::restricted_function();
}

pub fn main2() {
  let open_box = my::OpenBox { contents: "Public Information"};
  println!("The open box contains {}", open_box.contents);

  // error - no public access
  // let closed_box = my::ClosedBox { contents: "Private Information" };

  // use public constructor
  let _closed_box = my::ClosedBox::new("Private Information");

  // error - no public access
  // println!("The closed box contains {}", closed_box.contents);
}  

use deeply::nested::function as other_function;

mod deeply {
  pub mod nested {
    pub fn function() {
      println!("called `deeply::nested::function`");
    }
  }
}

pub fn main3() {
  other_function();

  println!("entering block");
  {
    // local shadow in this block
    use crate::modules::deeply::nested::function;

    function();

    println!("leaving block");
  }

  function();
}

mod cool {
  pub fn function() {
    println!("called `modules::cool::function`");
  }
}

pub fn main4() {
  my::indirect_call();
}
