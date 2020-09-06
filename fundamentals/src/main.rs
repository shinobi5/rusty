#![allow(dead_code)] // disable dead code

mod types_vars;
mod control_flow;
mod data_structures;

// global vars
// const A_GLOBAL_CONSTANT:u8 = 42; // no fixed address
// static A_GLOBAL_STATIC:i32 = 123;

fn main() {
  // types_vars::fundamental_data_types::fundamental_data_types();
  // types_vars::operators::operators();
  // types_vars::scope_shadowing::scope_shadowing();
  // types_vars::stack_heap::stack_heap();
  // control_flow::if_control::if_control();
  // control_flow::while_loop::while_loop();
  // control_flow::for_loop::for_loop();
  // control_flow::match_statement::match_statement();
  // control_flow::combination_lock::combination_lock();
  // data_structures::structs::structs();
  data_structures::enums::enums();
}
