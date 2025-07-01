fn main() {
    let x = 3;

    module(x);
    module_2(2);
}

fn module(_a: i32) {
    if _a % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
    println!("{}", _a);
}

fn module_2(_b: i32) {
    if _b % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}
