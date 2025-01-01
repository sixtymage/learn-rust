// rustc src/main.rs --extern rary=../rary/target/debug/library.rlib && ./main.exe
pub fn main() {
  rary::public_function();

  // error -- private
  //rary::private_function();

  rary::indirect_access();
}
