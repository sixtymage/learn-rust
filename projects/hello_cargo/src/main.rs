mod formatted_print;
mod hello;
mod point2d;
mod primitives;
mod vector;
mod city;
mod colour;
mod operators;
mod tuples;
mod arrays;
mod structures;
mod enums;
mod linked_list;
mod constants;
mod variables;
mod types;
mod conversion;
mod expressions;
mod flow_of_control;
mod functions;
mod functions_methods;
mod functions_closures;
mod closures_capturing;
mod closures_inputs;
mod closures_type_anonymity;
mod closures_functions;
mod closures_outputs;
mod closures_examples1;
mod closures_examples2;
mod functions_hof;
mod functions_diverging;
mod modules;
mod attributes;

fn main() {
  hello::hello_world();
  formatted_print::formatted_print();
  formatted_print::debug_print();
  formatted_print::custom_display();
  point2d::display_point2d();
  vector::display_list();
  city::display_city();
  colour::display_colour();
  primitives::declare_primitives();
  operators::simple_operations();
  tuples::exercise_tuples();
  arrays::exercise_array();
  structures::exercise_structs();
  enums::exercise_enum();
  enums::exercise_use_enums();
  enums::exercise_cstyle_enums();
  linked_list::exercise_list();
  constants::exercise_constants();
  variables::exercise_bindings();
  variables::exercise_mutability();
  variables::exercise_scope();
  variables::exercise_shadowed();
  variables::declare_first();
  variables::freezing();
  types::casting();
  types::literals();
  types::inference();
  types::aliasing();
  conversion::from_and_into();
  conversion::exercise_from();
  conversion::exercise_into();
  conversion::exercise_try_from_into();
  conversion::exercise_display();
  conversion::exercise_from_str();
  expressions::exercise_expressions();
  flow_of_control::if_else();
  flow_of_control::exercise_loop();
  flow_of_control::exercise_inner_break();
  flow_of_control::exercise_loop_return();
  flow_of_control::exercise_while();
  flow_of_control::for_and_range1();
  flow_of_control::for_and_range2();
  flow_of_control::for_with_iter();
  flow_of_control::for_with_into_iter();
  flow_of_control::for_with_iter_mut();
  flow_of_control::exercise_match1();
  flow_of_control::exercise_match2();
  flow_of_control::destructure_match();
  flow_of_control::destructure_array_slice();
  flow_of_control::destructure_enum();
  flow_of_control::destructure_pointers();
  flow_of_control::destructure_struct();
  flow_of_control::match_guards();
  flow_of_control::match_guards2();
  flow_of_control::match_binding();
  flow_of_control::match_binding2();
  flow_of_control::if_let();
  flow_of_control::if_let2();
  flow_of_control::if_let3();
  flow_of_control::let_else();
  flow_of_control::while_let();
  functions::main();
  functions_methods::main();
  functions_closures::main();
  closures_capturing::main();
  closures_capturing::main2();
  closures_inputs::main();
  closures_type_anonymity::main();
  closures_functions::main();
  closures_outputs::main();
  closures_examples1::main();
  closures_examples2::main();
  functions_hof::main();
  functions_diverging::main();
  modules::main();
  modules::main2();
  modules::main3();
  modules::main4();
  attributes::main();
  attributes::main2();
 
}

