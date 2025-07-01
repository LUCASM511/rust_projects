fn main() {
    'outer: for i in 1..6 {
        println!("Module table {}", i);
        'inner: for j in 1..6 {
            if i == 2 {
                continue 'outer;
            }
            if j == 3 {
                continue 'inner;
            }
            println!("{} % {} = {}", i, j, i % j);
        }
    }
}
