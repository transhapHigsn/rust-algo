/*
  Convert minutes into digital clock output. Can assume minutes will be less than 24*60.
*/

fn digital_clock(minutes: i32) -> String {
  if minutes > 1439 {
    panic!("Invalid minutes. {}", minutes);
  }

  let hours = minutes/60;
  let remaining_minutes = minutes % 60;

  let output = format!("{}:{:0>2}", hours, remaining_minutes);

  output
}

fn main() {
  let minutes: i32 = 1145;
  let result = digital_clock(minutes);

  println!("{}", result);
}

