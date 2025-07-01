use std::io;

fn main() {
    println!("Enter with one number:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error! Enter with a number");

    let input: u32 = match input.trim().parse {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("Your input: {input}");
}
