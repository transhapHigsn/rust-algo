/*

column, reversed, sort_manner=(numerical, lexical)

Input:

3
22 33
44 56
123 3
2 false lexical

Output:

123 3
22 33
44 56

*/

#[allow(dead_code)]
enum SortManner {
  Numerical,
  Lexical
}

fn get_cmp<'b>(row: &'b str, key: usize) -> &'b str {
  row.split_whitespace().collect::<Vec<&str>>()[key]
}

fn string_key_sort<'b, 'a>(arr: &'a [&'b str], reversed: bool, manner: SortManner, key: usize) -> Vec<&'b str> {
  let mut str_arr: Vec<(&str, &str)> = arr.iter().map(|&row| (row, get_cmp(row, key))).collect::<Vec<(&str, &str)>>();

  match manner {
    SortManner::Numerical => str_arr.sort_by_key(|&row| row.1.parse::<u32>().unwrap()),
    SortManner::Lexical => str_arr.sort_by_key(|&row| row.1),
  };

  if reversed {
    str_arr.reverse();
  }
  let result: Vec<&str> = str_arr.iter().map(|row| row.0).collect::<Vec<&str>>();
  result
}

fn main() {
  let input = vec!["22 33", "44 56", "123 3"];
  let result = string_key_sort(&input[..], true, SortManner::Numerical, 0 as usize);

  println!("{:?}", result) ;
}

