// arr = [1, 9, 3, 0, 18, 5, 2, 4, 10, 7, 12, 6]
// band formed = {0, 1, 2, 3, 4, 5, 6, 7} {9, 10} {11} {18}
// output is length of largest band.

// brute force solution is sorting and then finding band. O(n log n)
// optimised solution: O(n)

// unordered set for lookup in O(1)

use std::collections::HashSet;
use std::cmp::max;
use std::iter::FromIterator;

fn longest_band(num: &[i32]) -> u32 {
//   let mut set = HashSet::new();

  let set: HashSet<i32> = HashSet::from_iter(&mut num.iter().cloned());

//   for n in &num {
//     set.insert(n);
//   }

  let mut val = 1;

  // outer loop runs: n times
  for n in num {
    let parent = n - 1;
    if !set.contains(&parent) {
      let mut next_no = n + 1;
      let mut count = 1;

      // inner loop only runs n times across all iterations.
      while set.contains(&next_no) {
        next_no += 1;
        count += 1;
      }

      val = max(count, val);
    }
  }

  val
}

fn main() {
//   let arr = vec![1, 9, 3, 0, 18, 5, 2, 4, 10, 7, 12, 6];
  let arr = vec![1, 5, 9, 0, 3, 6, 2, 4, 10];

  let result = longest_band(&arr);
  println!("{}", result);
}

