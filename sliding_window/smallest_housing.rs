/*
  Given a sequence of plot sizes and required total plot size, find smallest contagious plots sequence whose area size is equal to required size.

  Input:
    - [1, 3, 2, 1, 4, 1, 3, 2, 1, 1], required size is 8

    - [8, 9, 10, 4, 7, 6, 2], required size is 8

  Output
    - (4, 6)

    - (0, 0)

*/


fn smallest_housing(arr: &[u32], required: u32) -> Option<(usize, usize)> {
  let mut i :usize = 0;
  let mut j :usize = 0;
  let mut cs = 0;

  let mut smallest_size: usize = arr.len();
  let mut ss: Option<(usize, usize)> = None;

  while j < arr.len() {
    cs += arr[j];
    j += 1;

    while cs > required && i < j {
      cs -= arr[i];
      i += 1;
    }

    if cs == required {
      let curr_size = j-1-i;
      if curr_size < smallest_size {
        smallest_size = curr_size;
        ss = Some((i, j-1));
      }
    }

  }

  ss
}

fn main() {
  let arr = vec![1, 3, 2, 1, 4, 1, 3, 2, 1, 1];
//   let arr = vec![8, 9, 10, 4, 7, 6, 2];
  let required_sum = 8;

  let result = smallest_housing(&arr[..], required_sum);
  match result {
    Some(val) => println!("{:?}", val),
    None => println!("No such option.")
  }
}

