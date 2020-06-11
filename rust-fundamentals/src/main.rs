#![allow(dead_code)] // disable dead code
mod hello_world;
mod fundamental_data_types;
mod operators;
mod scope_shadowing;
mod stack_heap;
mod control_flow;

// global vars
// const A_GLOBAL_CONSTANT:u8 = 42; // no fixed address
// static A_GLOBAL_STATIC:i32 = 123;

fn main() {
  // hello_world::hello_world();
  // fundamental_data_types::fundamental_data_types();
  // operators::operators();
  // scope_shadowing::scope_shadowing();
  // stack_heap::stack_heap();
  // control_flow::if_flow();
  // control_flow::while_loop_flow();
  // control_flow::for_loop_flow();
  // control_flow::match_statement_flow();
  control_flow::combination_lock_flow();
}
