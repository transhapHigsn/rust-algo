use std::convert::TryInto;

fn increment_by_1(alpha: &str) -> String {
    let alphabet = (b'A'..=b'Z')
        .filter_map(|c| {
            let c = c as char;                            
            if c.is_alphabetic() { Some(c) } else { None }
        })          
        .collect::<Vec<_>>();

    let mut carry = 1;
    let mut final_string = "".to_string();
    let mut next_idx: usize;
    for ch in alpha.chars().rev() {
        match alphabet.iter().position(|&x| x == ch) {
            Some(index) => {
                next_idx = index;
                if carry == 1 {
                    next_idx = index + 1;
                    carry = 0;
                }

                if next_idx > 25 {
                    next_idx = 0;
                    carry = 1;
                }

                final_string = format!("{}{}", alphabet[next_idx], final_string);
            },
            None => {
                panic!("Invalid character");
            }
        }
    }

    if carry > 0 {
        panic!("String size exceeded.");
    }

    final_string
} 

fn next_code(code: &str, no_of_alphabets: usize, no_of_digits: usize) -> String {
    let alpha_part = &code[0..no_of_alphabets];

    let mut digit = (&code[no_of_alphabets..])
                    .trim()
                    .parse::<i32>()
                    .expect("unable to parse digit part");
    
    digit = digit + 1;
    if digit == 10_i32.pow(no_of_digits.try_into().unwrap()) {
        format!(
            "{}{}",
            increment_by_1(&alpha_part),
            "0".repeat(no_of_digits)
        )
    } else {
        format!(
            "{}{}",
            alpha_part.to_string(),
            digit
        )
    }
}

fn main() {
    let mut line = String::new();
    println!("Enter input :");
    std::io::stdin().read_line(&mut line).unwrap();

    let vec = line.split(" ").collect::<Vec<&str>>();

    let n_code = next_code(
        vec[0],
        vec[1].trim().parse::<usize>().expect("Unable to parse!"),
        vec[2].trim().parse::<usize>().expect("Unable to parse!")
    );
    println!("{}", n_code);
}
