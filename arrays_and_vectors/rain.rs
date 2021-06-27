use std::cmp::{min, max};

fn rain(num: &[u32]) -> u32 {
  let n = num.len();

  if n <= 2 {
    return 0;
  }

  let mut left_vec: Vec<u32> = vec![0; n];
  let mut right_vec: Vec<u32> = vec![0; n];

  left_vec[0] = num[0];
  right_vec[n-1] = num[n-1];

  for i in 1..n {
    left_vec[i] = max(left_vec[i-1], num[i]);
    right_vec[n-i-1] = max(right_vec[n-i], num[n-i-1]);
  }

  let mut water = 0;
  for i in 0..n {
    water += min(left_vec[i], right_vec[i]) - num[i];
  }
  water
}

fn main() {
//   let num = vec![0, 1,0, 2, 4, 1, 0, 3, 2, 1];
  let num = vec![0,1,0,2,1,0,1,3,2,1,2,1];
//   let num = vec![2, 3];
  let result = rain(&num);

  println!("{}", result);
}

