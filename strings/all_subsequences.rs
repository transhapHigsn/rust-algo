/*
  get all sorted subsequence of a pattern string. first sort by length, then sort lexographically.

  this can be done using two approaches: i) recusrsion and ii) bitmasking.
  current implementation uses recursion.
*/


fn all_subsequences(msg: &str, combination: String, combinations: &mut Vec<String>) {
//   println!("{:?} {:?} {:?}", msg, combination, combinations);
  if msg.is_empty() {
    combinations.push(combination);
    return;
  }

  let rem_str = msg.get(1..).unwrap();
  // this is for older implementation where msg was &[u8]
  //   let new_comb = combination.to_owned() + &String::from_utf8(vec![msg[0]]).unwrap();
  let new_comb = combination.to_owned() + msg.get(0..1).unwrap();
  all_subsequences(rem_str, new_comb, combinations);

  // there is trick here. combination is moved here.
  // so rearranging order of all_subsequences function call/new_comb value calculation will break code.
  // if you can fix it, then you are welcome to do it.
  all_subsequences(rem_str, combination, combinations);

}

fn main() {
  let msg: &str = "abcd";
  let combination: String = "".to_string();
  let mut combinations: Vec<String> = vec![];

  // find all subsequences
  all_subsequences(msg, combination, &mut combinations);

  // sort using comparator function.
  combinations.sort_by(|a, b| {
    if a.len() == b.len() {
      return a.cmp(b);
    }
    return a.len().cmp(&b.len());
  });

  println!("{:?}", combinations);
}

