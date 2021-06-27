/*
  input = [1, 2, 3, 4, 5]
  output = [120, 60, 40, 30, 24]

  naive solution:
  l = [1,    2,   6, 24, 120]
  r = [120, 120, 60, 20, 5]
  p = [1*120, 1*60, 2*20, 6*5, 24*1]

  output[i] = product of all other elements exclusing ith element in input.

  efficient solution:
  input = [1, 2, 3, 4, 5]

  i = 0
  temp = 1
  p[0] = 1

  i = 1
  p[1] = 1
  temp = 2

  i = 2
  p[2] = 2
  temp = 6

  i = 3
  p[3] = 6
  temp = 24

  i = 4
  p[4] = 24
  temp = 120


  iterate from back now
  i = 4, temp = 1

  i = 4
  p[4] = p[4]*temp = 24*1 = 24
  temp = 1*5 = 5

  i = 3
  p[3] = p[3] * temp = 6 * 5 = 30
  temp = 5*4 = 20

  i = 2
  p[2] = p[2]*temp = 2 * 20 = 40
  temp = 60

  i = 1
  p[1] = p[1] * temp = 1*60 = 60
  temp = 60 * 2 = 120

  i = 0
  p[0] = p[0] * temp = 1 *120 = 120
  temp = 120*1 = 120

  [1, 1, 6, 24, 120]

  note: do not use division.
*/


// time complexity O(n)
// space complexity O(n)
fn naive_array_products(num: &[u32]) -> Vec<u32> {
  let size = num.len();
  let mut left_arr = vec![1 as u32; size];
  let mut right_arr = vec![1 as u32; size];
  let mut product_arr = vec![1 as u32; size];

  let mut i = 0;
  left_arr[i] = num[i];
  right_arr[size-i-1] = num[size-i-1];

  i += 1;
  while i < size  {
    left_arr[i] = num[i] * left_arr[i-1];
    right_arr[size-i-1] =  num[size-i-1] * right_arr[size-i];
    i += 1;
  }

  product_arr[0] = 1 * right_arr[1];
  product_arr[size-1] = 1 * left_arr[size-2];
  for i in 1..size-1 {
    product_arr[i] = left_arr[i-1] * right_arr[i+1];
  }
  product_arr
}

// time complexity O(n)
// space complexity O(1)
fn optimised_array_products(num: &[u32]) -> Vec<u32> {
  let size = num.len();
  let mut product = vec![1 as u32; size];

  let mut temp = 1;
  for i in 0..size {
    product[i] = product[i] * temp;
    temp *= num[i];
  }

  temp = 1;
  for i in (0..size).rev() {
    product[i] = product[i] * temp;
    temp *= num[i];
  }

  product
}

fn main() {

//   let num = [1, 2, 3, 4, 5];
  let num = [1, 5, 2, 7, 3];
  // [210, 42, 105, 30, 70]
  let result = optimised_array_products(&num[..]);

  for i in result {
    print!("{},", i);
  }
  println!("");
}

