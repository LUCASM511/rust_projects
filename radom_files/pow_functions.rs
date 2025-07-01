fn main() {
    let a = 2;
    let b = 4;

    let result = (i32::pow(a, 3) + i32::pow(b, 3)) + (3 * a * b * (a + b));

    println!("{}", result);
}
