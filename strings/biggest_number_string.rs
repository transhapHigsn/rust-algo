/*
  Given a vector of numbers, generate largest number that can be formed using those numbers.
  Restrictions: cannot change ordering of numbers within numbers, but can ordering on numbers in vector itself.

  Input:
    - [10, 11, 20, 30, 3, 2, 1]

    - [310, 4, 50, 20, 19, 1]

    - [10, 11, 20, 31, 3, 1, 320]

  Output:
    - 33022011110

    - 50431020191

    - 3203132011110

  Solution:
    [(10, 1), (11, 11), (20, 2), (30, 3), (3, 3), (2, 2), (1, 1)]
    [(3, 3), (30, 3), (2, 2), (20, 2), (11, 11), (1, 1), (10, 1)]


  Opinion:
   - I don't think this is the best possible solution for this. Will update once i have optimised solution for this.

  Time complexity of current solution: O(n*n) (should be able to solve this in O(n* log n) solution)
  Space complexity: O(n)'

*/

fn get_cmp(val: i32) -> String {
  let s = format!("{}", val);
  let val = s.trim_matches('0'); // is time complexity O(n) here?

  val.to_string()
}

fn biggest_number_string(num: &[i32]) -> String {
  // this function has O(n) space complexity as well.

  // this might be problematic logic here, it is O(n*n) time complexity
  let mut str_arr: Vec<(i32, String)> = num.iter().map(|&row| (row, get_cmp(row))).collect::<Vec<(i32, String)>>();

  // this is O(n*log n) time complexity
  str_arr.sort_by(|a, b| {
    if b.1 == a.1 {
      return a.0.cmp(&b.0);
    }
    return b.1.cmp(&a.1);
  });

  // how to join numbers and form string now.
  // this is O(n) operation.
  let mut s = String::new();
  for i in str_arr {
    s = s + &(i.0.to_string());
  }

  s
}

fn main() {
//   let num = vec![10, 11, 20, 31, 3, 1, 320];
  let num = [310, 4, 50, 20, 19, 1];
  let result = biggest_number_string(&num[..]);

  println!("{}", result);
}

