// [-1, 2, 3, 4, -2, 6, -8, 3]

use std::cmp::max;


// this is referred to as Kadane's algorithm.
// time complexity is O(n)
// space complexity is O(1)
fn maximum_subarray_sum(num: &[i32]) -> u32 {
  let mut max_so_far: u32 = 0;
  let mut max_ending_here: i32 = 0;

  for i in num {
    max_ending_here = max_ending_here + i;
    max_so_far = max(max_so_far as i32, max_ending_here) as u32;
    if max_ending_here < 0 {
      max_ending_here = 0
    }
  }
  max_so_far
}


fn main() {
//   let arr =  [-1, 2, 3, 4, -2, 6, -8 , 3];
//   let arr = [1];
//   let arr = [-1, -2, -3];
//   let arr = [-1, -2, 1];
//   let arr = [-1, -2, 4];
//   let arr = [];
//   let arr = [1, -2, -4, 4];
  let arr = [-2, -4, 4, -8, -6, 10];
  let result = maximum_subarray_sum(&arr[..]);

  println!("{}", result);
}

