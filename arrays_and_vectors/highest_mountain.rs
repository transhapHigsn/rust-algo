// find peaks, where condition is a[i-1] < a[i] < a[i+1];

use std::cmp::max;

fn highest_mountain(num: &[i32]) -> i32 {
   let mut i = 1;
   let mut largest = 0;
   while i <= num.len() - 2 {
     let mut count = 1;

     if num[i-1] < num[i] && num[i] > num[i+1] {
       let mut j = i;
       while j >= 1 && num[j] > num[j-1] {
         count += 1;
         j -= 1;
       }

       while i <= num.len() - 2 && num[i] > num[i+1] {
         count += 1;
         i += 1;
       }

       largest = max(largest, count);
     } else {
       i += 1;
     }
   }
   largest
 }

fn main() {
   let peaks: Vec<i32> = vec![5, 6, 4, 3, 2,1, 5, 6, 7, 8, 9,0, -1, 4, 3]; // 8
   // let peaks: Vec<i32> = vec![5,6, 1,2, 3, 4, 5,4, 3, 2, 0, 1, 2, 3, -2, 4]; // 9

   //                               9
   //                            8
   //                        7
   //   6                 6
   //5                  5
   //        4                                     4
   //         3                                       3
   //           2
   //             1
   //                                     0
   //                                        -1
   let highest = highest_mountain(&peaks);
   println!("{}", highest);
}

