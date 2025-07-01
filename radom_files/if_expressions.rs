fn main() {
    let language = "Rust";

    if language == "Rust" {
        println!("Learning Rust");
    } else if language == "Java" {
        println!("Learning Java");
    } else {
        println!("You are learning something else");
    }

    let language = "C";
    let language2 = "C++";

    if language == "C" {
        if language2 == "C++" {
            println!("You are learning C and C++ language");
        }
    } else {
        println!("You aren't learning this languages");
    }

    let learning_lan = "Bash";

    let result = if learning_lan == "Bash" {
        "Learning Bash"
    } else {
        "Not learning Bash"
    };

    println!("{}", result);

    let x = "Rust";

    let y: bool = if x == "Rust" { true } else { false };

    println!("x:{}", x);
    println!("y:{}", y);
}
