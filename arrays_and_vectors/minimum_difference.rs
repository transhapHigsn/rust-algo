
/*

    A[] = {l, 3, 15, 11, 2}
    B[] = {23, 127, 235, 19, 8}

    A = [1, 2, 3, 11, 15]
    b = [8, 19, 23, 127, 235]

*/

fn minimum_difference(arr_a: &[u32], arr_b: &[u32]) -> [u32; 2] {
    let [mut i, mut j] = [0; 2];

    let mut arr = [0; 2];
    let mut min_diff = i32::MAX;
    while i < arr_a.len() && j < arr_b.len() {
        let diff = (arr_a[i] as i32 - arr_b[j] as i32).abs();
        println!("arr_a:{} arr_b:{} diff:{} min_diff:{}", arr_a[i], arr_b[j], diff, min_diff);

        if min_diff > diff {
            min_diff = diff;
            arr[0] = arr_a[i];
            arr[1] = arr_b[j];
        }

        if arr_a[i] > arr_b[j] {
            j += 1;
        } else {
            i += 1;
        }
    }
    arr
}

fn main() {
//     let mut arr_a = [1, 3, 15, 11, 2];
//     let mut arr_b = [23, 127, 235, 19, 8];

    let mut arr_a = [23, 5, 10, 17, 30];
    let mut arr_b = [26, 134, 135, 14, 19];

    arr_a.sort();
    arr_b.sort();

    let result = minimum_difference(&arr_a[..], &arr_b[..]);

    for i in &result {
        print!("{},", i);
    }
    println!("");
}

