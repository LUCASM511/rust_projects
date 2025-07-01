// Suppress all warnings from casts with overflow
#![allow(overflowing_literals)]

fn main() {
    let a: i32 = 13;
    let b = (a as f64) / 2.0;

    println!("a: {}", a);
    println!("b: {}", b);

    // integer to char
    // only u8 work
    let integer: u8 = 65;
    let character = integer as char;

    println!("{}", character);

    // overflow casting
    println!("1000 as u8: {}", 1000 as u8);
    println!("-1 as a u8: {}", (-1i8) as u8);
    println!("1000 as i16: {}", 1000 as i16);
}
