use std::cmp::{max, min};

struct Solution {}

impl Solution {
    fn median_2(a: i32, b: i32) -> f64 {
        ( a + b ) as f64 / 2.0
    }
    
    fn median_3(a: i32, b: i32, c:i32) -> f64 {
        println!("Median3: {} {} {}", a, b, c);
        let median = a + b + c
                     - max(a, max(b, c))
                     - min(a, min(b, c));
        median as f64
    }
    
    fn median_4(a: i32, b: i32, c: i32, d: i32) -> f64 {
        println!("Median4: {} {} {} {}", a, b, c, d);
        let max_median = max( a, max( b, max( c, d ) ) );
        let min_median = min( a, min( b, min( c, d ) ) );
        ( a + b + c + d - max_median - min_median ) as f64 / 2.0
    }
    
    fn find_median(arr: &[i32]) -> (f64, usize) {
        let arr_len = arr.len();
        if arr_len == 0 {
            return (0.0_f64, 0);
        } else if arr_len == 1 {
            return (arr[0] as f64, 0);
        }
        
        if arr_len % 2 == 0 {
            let mid = arr_len / 2;
            ((arr[mid] + arr[mid-1]) as f64 / 2.0, mid - 1)
        } else {
            let mid = (arr_len + 1) / 2;
            (arr[mid-1] as f64, mid - 1)
        }
    }
    
    fn already_sorted_median(arr1: &[i32], arr2: &[i32]) -> f64 {
        let arr1_len = arr1.len();
        let arr2_len = arr2.len();
        
        let size_sum = arr1_len + arr2_len;
        if size_sum % 2 == 0 {
            let mid = size_sum / 2;
            if mid - 1 > arr1_len - 1 {
                return (arr2[mid-1-arr1_len] + arr2[mid-arr1_len]) as f64 / 2.0;
            } else if mid - 1 == arr1_len - 1 {
                return (arr1[mid-1] + arr2[0]) as f64 / 2.0;
            } else {
                return (arr1[mid-1] + arr1[mid]) as f64 / 2.0;
            }
        } else { 
            // [1,] [3, 4]
            let mid = (size_sum + 1) / 2;
            if mid - 1 > arr1_len - 1 {
                return arr2[mid-1-arr1_len] as f64;
            } else {
                return arr1[mid-1] as f64;
            }
        }
        
    }
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut arr1: &[i32];
        let mut arr2: &[i32];
        if nums1.len() > nums2.len() {
            arr2 = &nums1[..];
            arr1 = &nums2[..];
        } else {
            arr1 = &nums1[..];
            arr2 = &nums2[..];
        }
        
        let mut arr1_len = arr1.len();
        let mut arr2_len = arr2.len();
        
        loop {
            println!("{:?} {:?}", arr1, arr2);
            
            if arr1_len == arr2_len && arr1_len == 1 {
                return Solution::median_2(arr1[0], arr2[0]);
            }
           
            let (median1, mid1) = Solution::find_median(arr1);
            if arr2_len == 0 {
                return median1;
            }
            
            let (median2, mid2) = Solution::find_median(arr2);
            if arr1_len == 0 {
                return median2;
            }
            
            if arr1[arr1_len - 1] <= arr2[0] {
                return Solution::already_sorted_median(arr1, arr2);
            } else if arr2[arr2_len - 1] <= arr1[0] {
                return Solution::already_sorted_median(arr2, arr1);
            }

            if arr1_len == 1 {
                if arr2_len % 2 == 0 {
                    return Solution::median_3(arr2[mid2], arr2[mid2+1], arr1[0]);
                }

                return Solution::median_2(
                    arr2[mid2],
                    Solution::median_3(
                        arr2[mid2-1],
                        arr2[mid2+1],
                        arr1[0]
                    ) as i32
                );
            }
            
            if arr1_len == 2 {
                if arr2_len == 2 {
                    return Solution::median_4(arr1[0], arr1[1], arr2[0], arr2[1]);
                }
                
                if arr2_len % 2 != 0 {
                    return Solution::median_3(
                        arr2[mid2],
                        max(arr1[0], arr2[mid2 - 1]),
                        min(arr1[1], arr2[mid2 + 1])
                    );
                }
                
                return Solution::median_4(
                    arr2[mid2],
                    arr2[mid2+1],
                    max(arr1[0], arr2[mid2-1]),
                    min(arr1[1], arr2[mid2+2])
                );
            }
            
            println!("{:?} {} {} {:?} {} {}", arr1, mid1, median1, arr2, mid2, median2);
            
            if median1 == median2 {
                return median1;
            } else if median1 > median2 {

                arr1 = &arr1[..=mid1];
                arr2 = &arr2[mid2..];
            } else {
                arr1 = &arr1[mid1..];
                arr2 = &arr2[..=mid2];
            }
            
            arr1_len = arr1.len();
            arr2_len = arr2.len();
        }
    }
}

fn main() {
    // let result = Solution::find_median_sorted_arrays(vec![1], vec![3, 4]);
    // assert_eq!(3_f64, result);

    // let result = Solution::find_median_sorted_arrays(vec![5], vec![3, 4]);
    // assert_eq!(4_f64, result);

    // let result = Solution::find_median_sorted_arrays(vec![2, 3], vec![1]);
    // assert_eq!(2_f64, result);

    // let result = Solution::find_median_sorted_arrays(vec![3, 6], vec![9]);
    // assert_eq!(6_f64, result);

    // let result = Solution::find_median_sorted_arrays(vec![], vec![3, 4]);
    // assert_eq!(3.5_f64, result);

    // let result = Solution::find_median_sorted_arrays(vec![1, 2], vec![]);
    // assert_eq!(1.5_f64, result);

    // let result = Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]);
    // assert_eq!(2.5_f64, result);

    // let result = Solution::find_median_sorted_arrays(vec![1, 3], vec![2, 4]);
    // assert_eq!(2.5_f64, result);

    // let result = Solution::find_median_sorted_arrays(vec![1], vec![3]);
    // assert_eq!(2_f64, result);

    let result = Solution::find_median_sorted_arrays(vec![1, 2, 3], vec![2, 4, 7]);
    assert_eq!(2.5_f64, result);

    // let result = Solution::find_median_sorted_arrays(vec![1, 2, 5], vec![3, 4, 6]);
    // assert_eq!(3.5_f64, result);

    // let result= Solution::find_median_sorted_arrays(vec![1, 2, 3, 6, 9], vec![4, 5, 7, 8]);
    // assert_eq!(5_f64, result);
    
    // let result = Solution::find_median_sorted_arrays(vec![1, 2, 3, 6, 9], vec![4, 5, 7, 8, 10]);
    // assert_eq!(5.5_f64, result);

    // let result = Solution::find_median_sorted_arrays(vec![2], vec![1, 3, 4]);
    // assert_eq!(2.5_f64, result);

    // let result = Solution::find_median_sorted_arrays(vec![2], vec![1, 3, 4, 6]);
    // assert_eq!(3_f64, result);

    // let result = Solution::find_median_sorted_arrays(vec![2, 2, 4, 4], vec![2, 2, 4, 4]);
    // assert_eq!(3_f64, result);

    let result = Solution::find_median_sorted_arrays(vec![1, 3], vec![2, 4, 5, 6]);
    assert_eq!(3.5_f64, result);

    let result = Solution::find_median_sorted_arrays(vec![1, 3], vec![2, 4, 5, 6, 9]);
    assert_eq!(4_f64, result);

    // this test case fails
    let result = Solution::find_median_sorted_arrays(vec![1, 2, 4], vec![3, 5, 6, 7]);
    assert_eq!(4_f64, result);
}