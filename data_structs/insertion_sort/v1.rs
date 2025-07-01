fn main() {
    let array = vec![2, 4, 1, 5, 3];
    let sorted_array = insertion_sort(array);
    println!("{:?}", sorted_array);
}

fn insertion_sort(mut array: Vec<i32>) -> Vec<i32> {
    for i in 1..array.len() {
        let mut j = i;

        while j > 0 && array[j] < array[j - 1] {
            array.swap(j, j - 1);
            j -= 1;
        }
    }
    array
}
