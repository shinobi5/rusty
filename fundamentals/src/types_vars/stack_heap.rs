use std::mem;

struct Point {
  x: f64,
  y: f64
}

fn origin() -> Point {
  Point{ x: 0.0, y: 0.0 }
}

pub fn stack_heap() {
  let p1 = origin(); // stack
  let p2 = Box::new(origin()); // heap with Box::new()

  println!("p1 takes up {} bytes", mem::size_of_val(&p1));
  println!("p2 takes up {} bytes", mem::size_of_val(&p2));

  let p3 = *p2; // unboxing / relocating back to stack
  println!("{}", p3.x);
}
