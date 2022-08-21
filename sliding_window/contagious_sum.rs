/*
  Given a sequence of number and required sum, find all contagious sequence whose sum is equal to required sum.
  Numbers can be positive/negative.

  Input:
    - [1, 3, 2, 1, 4, 1, 3, 2, 1, 1], required sum is 8

    - [8, 9, 10, 4, 7, 6, 2], required sum is 8

  Output
    - [(2, 5), (4, 6), (5, 9)]

    - [(0, 0), (5, 6)]
*/

use std::convert::TryInto;


fn contagious_sum(arr: &[i32], required: u32) -> Vec<(usize, usize)> {
  let mut i :usize = 0;
  let mut j :usize = 0;
  let mut cs = 0;

  let mut ss: Vec<(usize, usize)> = vec![];
  let required_i32 = required.try_into().unwrap();

  while j < arr.len() {

    cs += arr[j];
    j += 1;

    while cs > required_i32 && arr[j] > 0 && i < j {
      println!("j:{} i:{} cs:{}", arr[j], arr[i], cs);
      cs -= arr[i];
      i += 1;
    }

    if cs == required_i32 {
      ss.push((i, j-1));
    }

  }

  ss
}

fn main() {
  // let arr = vec![1, 3, 2, 1, 4, 1, 3, 2, 1, 1];
  let arr: Vec<i32> = vec![8, 9, -10, -4, 7, -6, 2];
  let required_sum = 7;

  let result = contagious_sum(&arr[..], required_sum);
  println!("{:?}", result);
}

