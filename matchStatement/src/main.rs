fn main() {
    let evaluation = true;

    let value = match evaluation {
        true => 20,
        false => 40,
    };

    println!("The value is {value}");
}
