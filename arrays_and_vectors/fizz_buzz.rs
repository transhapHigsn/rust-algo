fn fizz_buzz(n: u32) {
  for i in 1..=n {
    match i {
      x if x % 15 == 0 => println!("FizzBuzz"),
      x if x % 3 == 0 => println!("Fizz"),
      x if x % 5 == 0 => println!("Buzz"),
      _ => println!("{}", i)
    }
  }
}

fn main() {
  fizz_buzz(30);
}

