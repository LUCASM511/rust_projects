fn main() {
    let array: [i32; 10] = [3, 4, 1, 2, 5, 7, 6, 8, 10, 9];
    let y = 1;

    linear_search(array, y);
}

fn linear_search(array: [i32; 10], i: i32) -> i32 {
    for x in array.iter() {
        if i == *x {
            println!("The number {} was found, on position {} in array", i, x);
        }
    }
    -1
}
