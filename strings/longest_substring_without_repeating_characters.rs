use std::collections::HashMap;
use std::convert::TryInto;

pub fn length_of_longest_substring(s: &[u8]) -> i32 {
    let mut map = HashMap::new();
    
    let mut i = 0;
    let mut j = 1;
    let mut length;
    let mut max_length = 0;

    map.insert(s[i], 0);
    
    while j < s.len() {
        match map.get(&s[j]) {
            Some(val) => {
                if *val >= i {
                    i = *val + 1;
                } else {
                    map.insert(s[j], j);
                    j = j + 1;
                }
            },
            None => {
                map.insert(s[j], j);
                j = j + 1;
            },
        };

        length = j - i;
        if max_length < length {
            max_length = length;
        }
    }

    if max_length == 0 {
        length = j - i;
        if max_length < length {
            max_length = length;
        }
    }

    max_length.try_into().unwrap()
}

fn main() {
    // let s1 = "codingminutes";
    // let s1 = "abcadc";
    // let s1 = "aaaa";
    let s1 = " ";

    println!("{:#?}", s1.as_bytes());
  
    let result = length_of_longest_substring(s1.as_bytes());
    println!("{}", result);
  }
  
/*

a b c a d c
i j
{a: 0, b: 1}

a b c a d c
i   j
{a: 0, b: 1, c: 2}


a b c a d c
i     j
{a: 3, b: 1, c: 2}
3 - 0 = 3

a b c a d c
  i     j
{a: 3, b: 1, c: 2, d: 4}
3

a b c a d c
  i       j
{a: 3, b: 1, c: 5, d: 4}
5 - 1 = 4

a b c a d c
      i     j
{a: 3, b: 1, c: 5, d: 4}
6 - 3 = 3

*/