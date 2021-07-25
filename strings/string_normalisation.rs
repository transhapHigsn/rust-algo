/*
  Given a sentence with words separated by spaces. Objective is to return a copy of this sentence where all words:
  - first character is capital letter
  - the rest of characters are lower case.

  Input:

  - This IS so MUCH fun.

  Output:

  - This Is So Much Fun.

*/


fn normalisation(message: &str) -> String {
  let mut is_last_element_space: bool = true;
  let mut str_vec: Vec<u8> = vec![];

  for ch in message.chars() {
    if ch.is_whitespace() {
      is_last_element_space = true;
      str_vec.push(ch as u8);
    } else {
      let mut new_ch: char = '\0';

      // this is only valid if we are only talking about english alphabets,
      // since that is a constraint on the question, using this logic.
      // would love to have a better logic here.
      if is_last_element_space {
        for i in ch.to_uppercase() {
          new_ch = i;
        }
      } else {
        for i in ch.to_lowercase() {
          new_ch = i;
        }
      }

      str_vec.push(new_ch as u8);

      is_last_element_space = false;
    }
  }

  String::from_utf8(str_vec).unwrap()
}

fn main() {
  let message = String::from("This is so much fun!");
  let result = normalisation(&message);

  println!("{}", result);
}
