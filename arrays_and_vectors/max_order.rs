/*
  Question statement:

   You are given number of order placed per minute at amazon like
   [1, 2, 7, 3, 6, 5, 13, 8]
   [2, 7, 7, 6, 6, 13, 13]
   [7, 7, 7, 6, 13, 13]
   # [7, 7, 7, 13, 13]

    ^^ this is brute force solution it works in O(m*n)
   for some analytical purpose you have to implement an algorithm which can give the maximum number of orders placed per minute in every interval of n minute.

   for example, if n=3 output would be
   output = [7, 7, 7, 6, 13, 13]
*/

use std::cmp::max;

fn maximum_number(num: Vec<i32>, n: u32) -> Vec<i32> {
  let mut max_as_of_now: i32 = num[0];

  for i in 0..n {
    for j in 1..num.len() - 1 {
      max_arr.push(max());
    }
    max_as_of_now = max(max_as_of_now, num[i]);
  }

  let mut max_arr = vec![max_as_of_now];

  let mut j = 0;
  for i in n as usize..num.len() {
    max_as_of_now = max(num[i], max_as_of_now);
    max_arr.push(max(num[i], max_as_of_now));
  }

  max_arr
}

fn main() {

  // this does not works.
  // optimal solution should work in O(m log n)
  // where m is length of array
  // and n is interval.
  // this requires segment tree implementation so the solution would run in O(m log n)
  let arr = vec![1, 2, 7, 3, 6, 5, 13, 8];
  let result = maximum_number(arr, 3);
  for i in result {
    print!("{}, ", i);
  }
}

