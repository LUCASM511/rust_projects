fn main() {
    for x in 0..10 {
        println!("{}", x);
    }

    let _array = ['a', 'b', 'c', 'd', 'e'];

    for x in _array {
        println!("{} {:?}", x, x);
    }

    // To count how many times the loop has already executed
    // Use the .enumerate() function
    for (count, variable) in (5..9).enumerate() {
        println!("count = {} enumerate = {}", count, variable);
    }

    for i in 0..5 {
        if i % 4 == 0 {
            println!("{}", i);
        }
    }

    for (count, variable) in (0..9).enumerate() {
        if count * 2 == 4 {
            println!("count = {} variable = {}", count, variable);
        }
    }
}
