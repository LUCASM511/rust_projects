fn main() {
    // This make to array two array with one million element each
    // Take a lot of memory
    let a = [0; 1_000_000]; // array with 1 million
    let b = a; // copy a to b => b = [0; 1_000_000]

    // Pointers in Rust
    // This transfer data without copy it
    let x = Box::new([0; 1_000_000]); // a contruct to allocate memory in the heap
    let b = a; // move to b
}
