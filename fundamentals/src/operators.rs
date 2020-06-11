pub fn operators() {
  // arithmetic
  let mut a = 2 + 3 * 4;
  println!("{}", a);
  a = a + 1; // no -- or ++
  a -= 2; // a = a -2

  println!("remainder of {} / {} = {}", a, 3, (a%3));

  let a_cubed = i32::pow(a, 3); // power of something
  println!("{} cubed is equal to {}", a, a_cubed);

  let b = 2.5; // floating point number
  let b_cubed = f64::powi(b, 3); // power of for integer/whole number
  let b_to_pi = f64::powf(b, std::f64::consts::PI); // power of for floating point number
  println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

  // bitwise
  let c = 1 | 2; // | OR & AND ^ XOR ! NOR
                 // 01 OR 10 = 11 == 3_10
  println!("1|2 = {}", c);

  let two_to_10 = 1 << 10;
  println!("2^10 = {}", two_to_10);

  // logical
  let pi_less_4 = std::f64::consts::PI < 4.0; // true
  println!("{}", pi_less_4);
  // > <= >= ==
  let x = 5;
  println!("{}", x);
  let x_is_5 = x == 5; // true
  println!("{}", x_is_5);
}