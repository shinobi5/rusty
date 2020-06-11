use std::mem;

pub fn fundamental_data_types() {
  // numbers
  let a:u8 = 123; // unsigned number of 8bits
  println!("a = {}", a); // print value of a

  // mutable (mut)
  let mut b:i8 = 0;
  println!("b = {}", b);
  b = 42;
  println!("b = {}", b);

  let mut c = 123456789; // 32bit signed i32
  println!("c = {}, size = {} bytes", c,  mem::size_of_val(&c));
  c = -1;
  println!("c = {} after modification", c);

  // types: i8, u8, i16, u16, i32, u32, i64, u64
  let z:isize = 123; // isize / usize
  let size_of_z = mem::size_of_val(&z);
  println!("z = {}, takes up {} bytes, {}-bit os", z, size_of_z, size_of_z * 8);

  let d:char = 'x'; // can be let d and rust will figure out that it"s a character
  println!("d = {}, size = {} bytes", d,  mem::size_of_val(&d));

  let e  = 2.5; // double-precision, 8 bytes or 64 bits, f64
  println!("d = {}, size = {} bytes", e,  mem::size_of_val(&e));

  // boolean: true / false
  let f = false;
  println!("f = {}, size = {} bytes", f,  mem::size_of_val(&f));
  let g = 4 > 0; // true
  println!("g = {}, size = {} bytes", g,  mem::size_of_val(&g));
}