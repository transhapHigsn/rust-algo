

fn get_chocolates_bought(
    price: i32,
    wrap_required: i32,
    money: i32,
    remaining_wrap: i32
) -> i32 {
    if money < price && remaining_wrap < wrap_required {
        return 0;
    }

    let choc_bought: i32 = money / price;
    let money_left: i32 = money % price;

    let choc_from_wrapper: i32 = (choc_bought + remaining_wrap) / wrap_required;
    let wrap_left: i32 = (choc_bought + remaining_wrap) % wrap_required;

    return choc_bought
            + choc_from_wrapper
            + get_chocolates_bought(
                price,
                wrap_required,
                money_left,
                choc_from_wrapper + wrap_left
            );
}


fn main() {
    let mut line = String::new();
    println!("Enter input :");
    std::io::stdin().read_line(&mut line).unwrap();

    let vec = line.split(" ").collect::<Vec<&str>>();

    let n_code = get_chocolates_bought(
        vec[0].trim().parse::<i32>().expect("Unable to parse!"),
        vec[1].trim().parse::<i32>().expect("Unable to parse!"),
        vec[2].trim().parse::<i32>().expect("Unable to parse!"),
        0
    );
    println!("{}", n_code);
}
