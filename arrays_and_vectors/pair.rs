use std::collections::HashSet;
use std::iter::FromIterator;

fn pair(num: &[u32], sum: u32) -> Option<Vec<u32>> {
  let mut occured: HashSet<u32> = HashSet::from_iter(&mut num.iter().cloned());

  for &n in num {
    let val = sum - n;
    if occured.contains(&val) {
      return Some(vec![n, val]);
    } else {
      occured.insert(n);
    }
  }
  None
}


fn main() {
  let num = vec![1, 2, 3, 7, 8];
  let pairs = pair(&num, 18);

  match pairs {
    Some(arr) => println!("{}, {}", arr[0], arr[1]),
    None => println!("No pair found")
  }
}

// #[cfg(test)]
// mod tests {
//   use super::*;

//   #[test]
//   fn find_pair() {
//     let num = vec![1, 2, 3, 7, 8];
//       assert_eq!(vec![3, 7], pair(&num, 10));
//   }
// }
