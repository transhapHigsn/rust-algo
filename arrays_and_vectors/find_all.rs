
fn find_all(sentence: &str, word: &str) -> Vec<usize> {
  let mut matches: Vec<usize> = vec![];
  for (pos, _) in sentence.match_indices(word) {
    matches.push(pos);
  }
  matches
}


fn main() {
  let sentence = String::from("I liked the movie, acting in movie was great!");
  let word = String::from("movies");

  let result = find_all(&sentence, &word);
  for i in result {
    print!("{}, ", i);
  }
  println!("");
}

