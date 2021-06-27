// sorting of the array
// iterate over entire array, and solve pair sum for remaining array.

fn triplet(num: &[u32], sum: u32) -> Vec<Vec<u32>> {
  // this uses 2-pointer approach.

  let mut result: Vec<Vec<u32>> = vec![];
  for i in 0..num.len() {
    let mut j = i+1;
    let mut k = num.len() - 1;

    while j < k {
      let current_sum = num[i] + num[j] + num[k];
      // println!("sum:{} current_sum:{} num[i]:{} num[j]:{} num[k]:{} i:{} j:{} k:{}", sum, current_sum, num[i], num[j], num[k], i, j, k);
      if current_sum == sum {
        result.push(vec![num[i], num[j], num[k]]);
        j += 1;
        k -= 1;
      } else if current_sum > sum {
        k -= 1;
      } else {
        j += 1;
      }
    }
  }
  result
}


fn main() {
  let mut num = vec![1, 2, 4, 5, 6, 7, 2, 0, 3, 8, 9];
  num.sort();

  let triplets = triplet(&num, 18);

  for t in triplets {
    for val in t {
      print!("{},", val);
    }
    println!("");
  }
}

