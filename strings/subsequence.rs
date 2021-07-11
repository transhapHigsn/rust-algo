/*

Input:

S1: codingminutes
S2: cines

Output:

True

Reason: all characters of S2 are present in S1.

*/


fn subsequence(s1: &[u8], s2: &[u8]) -> bool {
  if s1.is_empty() || s2.is_empty() {
    return false;
  }

  let mut j = s2.len() - 1;
  let mut i = s1.len() - 1;

  let mut count: i32 = j as i32;

  loop {
    if s1[i] == s2[j] {
      count -= 1;
      if j == 0 {
        // this means all characters are parsed in destination array.
        break;
      }
      j -= 1;
    }

    if i == 0 {
      // this means all characters are parsed in source array.
      break;
    }
    i -= 1;
  }

  count == -1
}

fn main() {
  let s1 = "codingminutes";
  let s2 = "cines";

  let result = subsequence(s1.as_bytes(), s2.as_bytes());
  println!("{}", result);
}

