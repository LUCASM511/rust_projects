use rand::Rng; // library to generete the random numbers
use std::cmp::Ordering; // library to compare something else
use std::io; // library for handle input

fn main() {
    println!("Guessing Game");

    // generete a radom number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secrete number is: {secret_number}");

    // create a infiniteloop
    loop {
        println!("Enter a number.");

        // save the input number
        let mut guess = String::new();

        // read the user input
        io::stdin().read_line(&mut guess).expect("Error");

        // convert the string user input to int
        // prevent the user type string input
        // and remove the white space
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // compare the random number to the use input namber
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
