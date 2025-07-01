#![allow(unused_variables)]

fn main() {
    let mut array: [i32; 5] = [10, 3, 7, 9, 2];

    let x = 0;
    let y = 0;

    println!("Unsorted array");

    println!("{:?}", array);

    for x in 0..array.len() - 1 {
        for y in 0..array.len() - x - 1 {
            if array[y] > array[y + 1] {
                let temp = array[y];
                array[y] = array[y + 1];
                array[y + 1] = temp;
            }
        }
    }

    println!("Sorted array");

    println!("{:?}", array);
}
