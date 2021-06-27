
fn minimum_swaps(num: &[[i32; 2]]) -> u32 {
  // time complexity of this function is O(n) + O(n) = O(n)
  // space complexity is O(1)
  let size = num.len();
  let mut visited = vec![false; size];

  let mut i = 0; // to iterate over vector
  let mut ans = 0; // stores number of swaps
  while i < size {
    let mut curr = num[i];
    if visited[i] {
      // println!("skipping num[i][0]:{} num[i][1]: {}", curr[0], curr[1]);
      i += 1;
      continue;
    }

    let mut cycle = 0;
    let mut ptr = curr[1] as usize;
    while !visited[ptr] {
      visited[ptr] = true;
      cycle += 1;
      curr = num[ptr];
      ptr = curr[1] as usize;
      // println!("i:{} 0:{} 1:{} ans:{}, cycle:{}", i, curr[0], curr[1], ans, cycle);
    }

    ans += cycle - 1;
    i += 1;
  }

  ans
}

fn main() {
  // time complexity is O(n) + O(n log n) + O(n) = O(n log n)
  // space complexity is O(n)

  let arr = vec![5, 4, 2, 1, 3];
//   let arr = vec![5, 4, 3, 2, 1];
//   let arr = vec![5, 2];
//   let arr = vec![0];
  // idea is to sort and then find cycles in it.
  // ans is sum of (cycle - 1)

  // this is to make vector or array in form [[elementA, indexA], [elementB, indexB],...]
  let mut result = arr.iter().enumerate().map(|(i, x)|
    [*x as i32, i as i32]
  ).collect::<Vec<[i32; 2]>>();

  // this sorts by first element in asecending order,
  // inorder to sort in descending order do following
  // result.sort_by(|a, b| b[0].cmp(&a[0]));

  result.sort_by(|a, b| a[0].cmp(&b[0]));
  let ans  = minimum_swaps(&result[..]);

  println!("{}", ans);
}

