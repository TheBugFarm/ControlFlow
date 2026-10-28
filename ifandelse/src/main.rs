fn main() {
    let season = "summer";
    if season == "summer" {
        println!("school is out");
    } else if season == "winter" {
        println!("Brrrr, soo cold");
    } else if season == "fall" {
        println!("Leaves are falling");
    } else {
        println!("Lots of rain");
    }

    even_or_odd(17);
    even_or_odd(100);
}

fn even_or_odd(num: i32) {
    let result = if num % 2 == 0 { "even" } else { "odd" };
    println!("the number is {result}");
}