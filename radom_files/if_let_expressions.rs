fn main() {
    let x = ("Water", "Apple", "Steel");

    if let ("Water", "Apple", "Steel") = x {
        println!("Wrote all the value in pattern to be matched with the scrutinee expressions");
    } else {
        println!("Value unmatched");
    }

    // if the the first value or second value matches, it can guess the third value
    if let ("Water", "Apple", s) = x {
        println!("Wrote all the value in pattern to be matched with the scrutinee expressions");
    } else {
        println!("Value unmatched");
    }

    // if the the first value matches, it can guess the third value
    if let ("Water", a, b) = x {
        println!("Wrote all the value in pattern to be matched with the scrutinee expressions");
    } else {
        println!("Value unmatched");
    }

    let x = ("Rust", "expert");

    // when the pattern is not matched
    if let ("Java", b) = x {
        println!("Wrote all the value in pattern to be matched with the scrutinee expressions");
    } else {
        println!("Value unmatched");
    }

    // no pattern is define
    if let _ = 10 {
        println!("Irrefutable if-let pattern is always executed");
    }
}
