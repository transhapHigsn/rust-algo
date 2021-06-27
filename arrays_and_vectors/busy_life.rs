/*
  Given a list with start and finish time of activities, find the maximum number of activities that can be performed.

  for example:
  input = [[7,9], [0,10], [4, 5], [8, 9]. [4, 10], [5, 7]]
  output = 3
  reason:
    1. [4, 5]
    2. [5, 7]
    3. [7, 9] or [8, 9]
*/


use std::cmp::max;


// time complexity is O(n log n)
fn busy_life(activity: &[[u32; 2]]) -> u32 {
  let size = activity.len();

  let mut max_activities = 0;
  let mut count = 1;
  for i in 0..size-1 {
    if activity[i][1] <= activity[i+1][0] {
      count += 1;
    } else {
      count = 1;
    }

    max_activities = max(count, max_activities);
  }
  max_activities
}


fn main() {
  let mut activities = [[7, 10], [0, 9], [5, 6], [8, 9], [4, 10], [6, 7]];

  activities.sort_by(|a,b| a[1].cmp(&b[1]) );

  for i in &activities {
    for j in i {
      print!("{}, ", j);
    }
    println!("");
  }

  let result = busy_life(&activities[..]);

  println!("{}", result);
}

