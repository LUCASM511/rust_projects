fn main() {
    let mut var = 1;
    let mut found = false;

    while !found {
        var = 1 + var;

        println!("{}", var);

        if var % 2 == 1 {
            found = true;
        }

        println!("Loop runs");
    }

    let mut var = 1;

    // use to run infinitely
    loop {
        var = 1 + var;
        println!("{}", var);

        // to stop the loop
        if var == 10 {
            break;
        }
    }
}
