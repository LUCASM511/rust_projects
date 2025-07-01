mod sub;

fn main() {
    sub::sub();

    let condition = true;
    let x = if condition { 5 } else { 6 };

    println!("{}", x);

    let condition = false;
    let x = if condition { 5 } else { 6 };

    println!("{}", x);

    let mut count = 0;

    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2;
        }
    };

    println!("{result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}
