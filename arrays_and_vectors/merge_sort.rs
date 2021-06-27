fn merge_sort(mut arr: Vec<i32>) -> Vec<i32> {
  let length = arr.len();
  if length <= 1 {
    return arr;
  }

  let mid: usize = length / 2;

  let mut left_arr = (&arr[..mid]).to_vec();
  let mut right_arr = (&arr[mid..]).to_vec();

  // was trying to implement mutable reference approach
  // not able to work it out as of now.
  // keeping it in case i find a better approach to solve this.
  // let (mut left_arr, mut right_arr) = arr[..].split_at_mut(mid);

  left_arr = merge_sort(left_arr);
  right_arr = merge_sort(right_arr);

  let [mut i, mut j, mut k] = [0; 3];

  while i < left_arr.len() && j < right_arr.len() {
    if left_arr[i] < right_arr[j] {
      arr[k] = left_arr[i];
      i += 1;
    } else {
      arr[k] = right_arr[j];
      j += 1;
    }
    k += 1;
  }

  while i < left_arr.len() {
    arr[k] = left_arr[i];
    i += 1;
    k += 1;
  }

  while j < right_arr.len() {
    arr[k] = right_arr[j];
    j += 1;
    k += 1;
  }

  arr
}

fn main() {
//   let l_arr = vec![1, 0, 8, 3, -1, 5, 7];
  let l_arr = vec![4, 9, 6, 2, -2, 10];
  let sorted_arr = merge_sort(l_arr);

  for i in sorted_arr {
    print!("{},", i);
  }
  println!("");
}

