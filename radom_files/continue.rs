fn main() {
    for x in 0..10 {
        if x == 5 {
            println!("Continue Statement");
            continue; // control goes to the start of the loop
        }
        println!("{}", x);
    }

    let mut var = 1;
    let mut found = false;

    while !found {
        var = 1 + var;
        println!("{}", var);

        if var == 5 {
            println!("Inside Continue While Statement");
            continue; // go back to start
        }

        if var == 10 {
            found = true;
        }
    }

    loop {
        let mut var1 = 1;
        var1 = 1 + var1;
        println!("{}", var1);

        if var1 == 5 {
            println!("Inside Continue Loop Statement");
            continue; // go back to start
        }

        // the code don't reach on this lines b
        if var1 == 10 {
            break;
        }
        break;
    }
}
