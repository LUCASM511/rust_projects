use std::io;

fn main() {
    let mut user = String::new();

    println!("Hobbies");
    println!("1 => Football");
    println!("2 => Handboll");
    println!("3 => Voleibol");

    io::stdin()
        .read_line(&mut user)
        .expect("Error! Cannot read the input");

    let user: i32 = user.trim().parse().expect("Type a number");

    match user {
        1 => println!("Football"),
        2 => println!("Handbool"),
        3 => println!("Voleibol"),
        _ => println!("You like another hobbies"),
    };
}
