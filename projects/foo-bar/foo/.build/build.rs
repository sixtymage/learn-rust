use std::env;

fn main() {
  read_vars();
}

fn read_vars() {
  let var_name = "CARGO";

  match env::var(var_name) {
    Ok(value) => {
      println!("{}: {}", var_name, value);
    },
    Err(_) => {
      println!("Could not read environment variable {}", var_name);
    }
  }
}