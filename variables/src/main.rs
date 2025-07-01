// constante variable
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The values of x is {x}");
    x = 6;
    println!("The values of x is {x}");

    // Shadowing
    let y = 2;
    let y = y + 3;

    {
        let y = y * 6;
        println!("Inner scope {y}");
    }

    println!("{y}");
}
