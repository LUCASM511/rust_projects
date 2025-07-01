fn main() {
    let i = sum(&mut 1);

    println!("{}", i);
}

fn sum(i: &mut i32) -> i32 {
    while *i < 10 {
        i += 1;
    }
    i
}
