
// everything being in a utf-8, makes this probelm non-trivial to modify string in place.
// not implementing this until i find a suitable solution for it.


fn space_20(sentence: &mut String) {
  if !sentence.is_ascii() {
    panic!("Invalid string");
  }

  let chars: Vec<usize> = sentence.char_indices().filter(|(_, ch)| ch.is_whitespace()).map(|(idx, _)| idx ).collect();
  let char_count = chars.len();
  if char_count == 0 {
    return;
  }

  let sentence_len = sentence.len();
  sentence.push_str(&"*".repeat(char_count*2)); // filling string with * so that bytes array becomes of required size.
  .
  unsafe {
    let bytes = sentence.as_bytes_mut();

    let mut final_idx = sentence_len + (char_count * 2) - 1;

    let mut i = sentence_len - 1;
    let mut char_ptr = char_count - 1;
    loop {
      if i != chars[char_ptr] {
        bytes[final_idx] = bytes[i];
        if final_idx == 0 {
          // all elements are filled.
          println!("all elements are filled.");
          break;
        }
        final_idx -= 1;

      } else {
        bytes[final_idx] = '0' as u8;
        bytes[final_idx - 1] = '2' as u8;
        bytes[final_idx - 2] = '%' as u8;

        // final_idx is of type usize cannot be less than 0.
        if final_idx < 3 {
          println!("all elements are filled at start.");
          break;
        }

        final_idx -= 3;

        // char_ptr is of type usize cannot be less than 0.
        if char_ptr > 0 {
          char_ptr -= 1;
        }
      }

      if i == 0 {
        // all elements are parsed.
        println!("all elements are parsed.");
        break;
      }

      i -= 1;
    }

  }

}


fn main() {
  let mut sentence = String::with_capacity(1000);
  sentence.push_str("  hello, how are you?");
//   sentence.push_str("hello, how  are you?");
//   sentence.push_str(" hello, how are you? ");
//   sentence.push_str("  ");
//   sentence.push_str("abcd");

  space_20(&mut sentence);
  println!("{}", sentence);
}
