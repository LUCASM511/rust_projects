fn main() {
    let mut w = 10;
    let z = &mut w; // z = 10
    *z = 20; // z = 20
}
