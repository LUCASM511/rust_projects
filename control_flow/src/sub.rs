pub fn sub() {
    let a = [10, 20, 30, 40, 50];

    let mut index = 0;

    while index < 5 {
        println!("Value {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("value {}", element);
    }

    // reverse loop
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
