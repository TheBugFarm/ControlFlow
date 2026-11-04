fn main() {
    println!("{}", color_to_number("red"));

    println!("{}", factorial(5));
    println!("{}", factorial_1(5));
}

fn color_to_number(color: &str) -> i32 {
    match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
}

fn factorial(mut num: i32) -> i32 {
    let mut fact = 1;
    while num > 0 {
        fact *= num;
        num -= 1;
    }
    fact
}

fn factorial_1(num: i32) -> i32 {
    if num == 1 {
        return 1;
    }
    num * factorial_1(num - 1)
}