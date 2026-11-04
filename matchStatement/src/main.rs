fn main() {
    /*let evaluation = true;

    let value = match evaluation {
        true => 20,
        false => 40,
    };

    println!("The value is {value}");*/
    miss();
    niss();
}
fn miss() {
    let number = 8;

    match number {
        2 | 4 | 6 | 8 => println!("{number} is even"),
        1 | 3 | 5 => println!("{number} is odd"),
        _ => println!("Unknown for now"),
    }
}
fn niss() {
    let number = 5;

    match number {
        value if value % 2 == 0 => println!("{value} is an even number"),
        x if x % 2 != 0 => println!("{x} is an odd number"),
        _ => unreachable!(),
    }
}