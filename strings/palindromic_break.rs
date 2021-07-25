/*

  i = "zabccbaz";
  o = "aabccbaz";

  i = "a";
  o = "";

  i = "aa";
  o = "ab";

  i = "aba";
  o = "abb";

*/

fn break_palindrome(message: &str) -> String {
  let message_len: usize = message.len();

  if message_len <= 1 {
    return "".to_string();
  }

  let mid: usize = message_len / 2;

  let mut is_found: bool = false;
  let mut arr: Vec<u8> = vec![];

  let mut i: usize = 0;
  const A: u8 = 'a' as u8;

  for ch in message.as_bytes() {
    arr.push(*ch);
    if i >= mid {
      i += 1;
      continue;
     }

    // only replace with `a` before mid length of string
    if !is_found && *ch != A && i <= mid - 1 {
      arr[i] = A;
      is_found = true;
    }

    i += 1;
  }

  // if no change is done, before reaching mid value of string
  // change last character to `b`.
  if !is_found {
    arr[message_len-1] = 'b' as u8;
  }

  String::from_utf8(arr).unwrap()
}

fn main() {
//   let message = String::from("abccba");
//   let message = String::from("a");
//   let message = String::from("aa");
//   let message = String::from("aba");
  let message = String::from("zabccbaz");

  let result = break_palindrome(&message);
  println!("{}", result);
}

