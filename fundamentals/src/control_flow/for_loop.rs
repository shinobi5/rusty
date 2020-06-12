pub fn for_loop() {
  for x in 1..11 {
    if x == 3 { continue; }; // exclude 3
    if x == 8 { break; }; // leave loop at 8 (up to 7)
    println!("x = {}", x);
  }

  for (pos, y) in (30..41).enumerate() {
    println!("{}: {}", pos, y);
  }
}