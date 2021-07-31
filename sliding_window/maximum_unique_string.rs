/*
  Given a string, find the longest string contatining only unique characters.

  Input
  - abracadabra

  Output
  - brac

*/

use std::collections::HashMap;

fn max_unique_string(msg: &str) -> String {
  let mut map: HashMap<u8, usize> = HashMap::new();

  let msg_bytes = msg.as_bytes();

  // these variables are used to loop over given string.
  let mut i: usize = 0;
  let mut j: usize = 0;

  // following variables are used to store result info.
  let mut start_idx: usize = 0;
  let mut end_idx: usize = 0;
  let mut max_length: usize = 0;

  while j < msg.len() {
    let ch: u8 = msg_bytes[j];

    match map.get_mut(&ch) {
      Some(x) => {
        /*
          Since all character occurences is stored in HashMap, it is possible that
          character in question is not part of current window. Therefore, below logic
          is used to handle occurences within current window.
        */
        if *x >= i {
          if j - i - 1 > max_length {
            max_length = j - i - 1;
            start_idx = i;
            end_idx = j - 1;
          }
          i = *x + 1 as usize;
        }

        // update latest position of character.
        *x = j;
      },
      None => {
        map.insert(ch, j);
      }
    }

    j += 1;
  }

  // if no repeating occurences is found at the end of the string,
  // adjust start_idx & end_idx.
  if max_length < j - i - 1 {
    start_idx = i;
    end_idx = j - 1;
  }

  String::from_utf8(msg_bytes[start_idx..=end_idx].to_vec()).unwrap()

}

fn main() {
//   let msg = "messageeexcvfg";
//   let msg = "abracadabrafg";
//   let msg = "uniq";
  let msg = "aaaa";
  let result = max_unique_string(&msg);

  println!("{}", result);
}

