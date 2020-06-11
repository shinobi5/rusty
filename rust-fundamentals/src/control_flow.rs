#![allow(dead_code)]

use std::io::stdin;

pub fn if_flow() {
  let temp = 35;

  if temp > 30 {
    println!("really hot outside");
  } else if temp < 10 {
    println!("really cold!");
  } else {
    println!("temperature is OK");
  }

  let day = if temp > 20 { "sunny" } else { "cloudy" };
  println!("today is {}", day);

  println!(
    "it is {}", 
    if temp > 20 { "hot" } else if temp < 10 { "cold" } else { "OK" }
  );

  println!(
    "it is {}", 
    if temp > 20 {
      if temp > 30 { "very hot" } else { "hot" } 
    } else if temp < 10 { "cold" } else { "OK" }
  );
}

pub fn while_loop_flow() {
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

pub fn for_loop_flow() {
  for x in 1..11 {
    if x == 3 { continue; }; // exclude 3
    if x == 8 { break; }; // leave loop at 8 (up to 7)
    println!("x = {}", x);
  }

  for (pos, y) in (30..41).enumerate() {
    println!("{}: {}", pos, y);
  }
}

pub fn match_statement_flow() {
  let country_code = 1001;
  let country = match country_code { // match statement
    44 => "UK",
    46 => "Sweden",
    7 => "Russia",
    1..=1000 => "Unknown", // =1000 means including 1000 (not just up to 1000)
    _ => "Invalid"
  };
  println!("the country with code {} is {}", country_code, country);
}

pub fn combination_lock_flow() {
  enum State {
    Locked,
    Failed,
    Unlocked
  }
  let code = String::from("1234");
  let mut state = State::Locked;
  let mut entry = String::new();

  loop {
    match state {
      State::Locked => {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
          Ok(_) => {
            entry.push_str(&input.trim_end());
          }
          Err(_) => continue
        }

        if entry == code {
          state = State::Unlocked;
          continue;
        }

        if !code.starts_with(&entry) {
          state = State::Failed;
        }
      }

      State::Failed => {
        println!("FAILED");
        entry.clear(); // makes empty string ""
        state = State::Locked;
        continue;
      }

      State::Unlocked => {
        println!("UNLOCKED");
        return;
      }
    }
  }
}