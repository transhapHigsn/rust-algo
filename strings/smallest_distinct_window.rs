/*

Given a string S, find the smallest window containing all the distinct characters from the string S.

For example:

Input:
S = "aabcbcdbcaaad"

Output:
"dbca"

Input:
S = "aaaa"

Output:
"a"

*/

use std::collections::HashMap;

fn smallest_window(chars: &[u8]) -> [u32; 2] {
    let mut map = HashMap::new();
    for i in chars {
        match map.get(&i) {
            Some(val) => {
                map.insert(i, *val + 1);
            },
            None => {
                map.insert(i, 0);
            },
        };
    }
    [0 as u32; 2]
}

fn main() {
    let input = "aabcbcdbcaaad";

    let result = smallest_window(input.as_bytes());
    println!("{:#?}", result);
}