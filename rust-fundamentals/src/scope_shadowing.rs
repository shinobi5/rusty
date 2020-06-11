pub fn scope_shadowing() { // scope created by function
  let a = 123;
  println!("outside a = {}", a);

  { // new scope created by curly braces
    let a = 777;
    println!("inside a = {}", a);

    let b = 456;
    println!("inside b = {}", b);
  }
}