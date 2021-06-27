

fn subarray_sort(mut num: Vec<i32>) -> [i32; 2] {
  let mut max: i32 = -1;
  let mut min: i32 = -1;

  let size = num.len();
  let mut i = 0;
  while i <= size as i32 - 2 {
    // if there is only one element

    // this is to access vector.
    let j = i as usize;

    if num[j] > num[j+1] {
      if min == -1 {
        min = j as i32;
        max = j as i32 + 1;
        let tmp = num[j];

        // swap value when not in order
        num[j] = num[j+1];
        num[j+1] = tmp;
      } else {
        max = j as i32 + 1;
        let tmp = num[j];

        // swap value when not in order
        num[j] = num[j+1];
        num[j+1] = tmp;
      }
    }
//     println!("min:{} max:{}", min, max);
    i += 1;
  }

  [min, max]
}

// fn subarray_sort_1(num: &[i32]) -> [i32; 2] {
//   // find maximum and minimum number out of place
//   // find their rightful condition that is the output.
//   [-1, -1]

//   let size = num.len();

//   let mut i = 1;
//   let min: i32 = -1;
//   while i < size {
//     if num[i-1] > num[i] {
//       min = num[i];
//       break;
//     }
//     i += 1;
//   }

//   let mut j = size - 2;
//   let mut max: i32 = -1;
//   while j >= 0 {
//     if num[j-1] > num[j] {
//       max = num[j];
//       break;
//     }
//     j -= 1;
//   }

// }

fn main() {
  let arr = vec![1, 2, 3, 4, 5, 8, 6, 7, 9, 10, 11];
//   let arr = vec![1, 2, 9, 4, 5, 6, 7, 8, 10, 3, 11];
//   let arr = vec![1, 2, 9, 4, 5, 6, 7, 8, 10, 11];
  //             0  1  2  3  4  5  6  7   8  9  10
//   let arr = vec![0, 1];
//   let arr = vec![1, 0];
//   let arr = vec![1];
  let result = subarray_sort(arr);

  for val in &result {
    print!("{},", val);
  }
  println!("");
}

