/* This programming show the basics Ownership with collections of String using box */
fn main() {
    let first = String::from("Ford"); // allocate the strin "Ford" in the heap
    let full = add_suffix(first); // this is called, then moves ownership from first to name
                                  // when ends the function return name, trasferring ownership of the string to full
    println!("{}", full);
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr."); // resize the string's heap allocation and it frees the original heap memory
    name
}
