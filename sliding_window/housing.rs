/*
  Given a sequence of plot sizes and required total plot size, find all contagious plots whose area size is equal to required size.
  Restriction: all values will be > 0.

  Input:
    - [1, 3, 2, 1, 4, 1, 3, 2, 1, 1], required size is 8

    - [8, 9, 10, 4, 7, 6, 2], required size is 8

  Output
    - [(2, 5), (4, 6), (5, 9)]

    - [(0, 0), (5, 6)]

*/


fn housing(arr: &[u32], required: u32) -> Vec<(usize, usize)> {
  let mut i :usize = 0;
  let mut j :usize = 0;
  let mut cs = 0;

  let mut ss: Vec<(usize, usize)> = vec![];

  while j < arr.len() {

    cs += arr[j];
    j += 1;

    while cs > required && i < j {
      cs -= arr[i];
      i += 1;
    }

    if cs == required {
      ss.push((i, j-1));
    }

  }

  ss
}

fn main() {
//   let arr = vec![1, 3, 2, 1, 4, 1, 3, 2, 1, 1];
  let arr = vec![8, 9, 10, 4, 7, 6, 2];
  let required_sum = 8;

  let result = housing(&arr[..], required_sum);
  println!("{:?}", result);
}

