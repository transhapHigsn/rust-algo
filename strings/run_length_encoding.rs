/*
  Write a function to perform run length encoding. If generated string is longer than input string,
  return original string.

  Input:
    - bbbaaaadexxxxxx

    - abc

  Output:
    - b3a4d1e1x6

    - abc

*/


fn run_length_encoding(message: &str) -> String {
  let mut curr_ch: char = '\0';
  let mut curr_count: i32 = 0;
  let mut s = String::new();

  for i in message.chars() {
    if curr_ch == '\0' {
      curr_ch = i;
      curr_count = 0;
    }

    if i != curr_ch {
      s = s + &(format!("{}{}", curr_ch, curr_count));
      curr_ch = i;
      curr_count = 0;
    }

    curr_count += 1;
  }

  if curr_ch != '\0' {
    s = s + &(format!("{}{}", curr_ch, curr_count));
  }

  if message.len() <= s.len() {
    return message.to_string();
  }
  s
}

fn main() {
//   let message = String::from("aaabbbcdhs");
//   let message = String::from("");
  let message = String::from("abc");
  let result = run_length_encoding(&message);

  println!("{}", result);
}

