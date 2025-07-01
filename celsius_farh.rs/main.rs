#![allow(unused_variables, unused_assignments, unused_mut)]

mod fibo;

const CELSIUS: f32 = 20.0;
const FAHR: f32 = 52.0;

fn main() {
    println!("Temperatura in  °C: {}", CELSIUS);
    println!("Temperatura in  °F: {}", FAHR);

    let _farh = (CELSIUS * 1.8) + 32.0;
    let _celsius = (FAHR - 32.0) * (5.0 / 9.0);

    println!("Farh: {} Celsius: {}", _farh, _celsius);
}
