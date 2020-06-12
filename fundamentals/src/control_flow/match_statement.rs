pub fn match_statement() {
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