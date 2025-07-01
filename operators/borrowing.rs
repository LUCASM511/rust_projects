#![allow(unused_variables, unused_mut, unused_assignments)]

fn main() {
    // shared borrow (&)
    let a = 10;
    let b = &a;

    // mutable borrow (&mut)
    let mut x = 10;
    let y = &mut x;

    let mut b = &a;
    b = &14;
    let a = &mut b;

    let mut w = 10;
    let mut z = &mut w; // z = 10
    *z = 0; // z = 0
}
