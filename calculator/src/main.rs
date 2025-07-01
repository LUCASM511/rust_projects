fn op(x: i32, operator: char, y: i32) {
    match operator {
        '+' => {
            println!("{}", x + y);
        }
        '-' => {
            println!("{}", x - y);
        }
        '*' => {
            println!("{}", x * y);
        }
        '/' => {
            if y == 0 {
                println!("Division by 0");
            } else {
                println!("{}", x / y);
            }
        }
        '%' => {
            if y == 0 {
                println!("Mod 0 is undefined");
            } else {
                println!("{}", x % y);
            }
        }
        _ => println!("Invalid operator"),
    }
}

fn main() {
    op(2, '+', 1);
    op(2, '-', 1);
    op(2, '*', 1);
    op(2, '/', 1);
    op(2, '%', 1);
}
