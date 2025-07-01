use std::{collections::LinkedList, fmt::Debug};

fn main() {
    let mut list = LinkedList::new();

    list.push_back('k');
    list.push_back('l');

    print!("{:?}", list);
}
