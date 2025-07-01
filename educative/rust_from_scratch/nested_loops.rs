fn main() {
    for i in 1..5 {
        println!("Multiplication table {}", i);
        for j in 1..5 {
            println!("{} * {} = {}", i, j, i * j);
        }
    }
}
