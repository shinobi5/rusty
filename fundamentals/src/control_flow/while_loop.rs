pub fn while_loop() {
  let mut x = 1;

  while x < 1000 {
    x *= 2;
    if x == 64 { continue; } // skip 64 if it exists
    println!("x = {}", x);
  }

  let mut y = 1;
  
  loop {
    y *= 2;
    println!("y = {}", y);
    if y == 1 << 10 { break; } // 2 to the power of 10 i.e. 1 shifted 10 places to the left
  }
}