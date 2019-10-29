use algorithms::tree::BinaryHeap;
use std::collections::HashMap;

fn main() {

    let mut bh:BinaryHeap<i32> = BinaryHeap::from_array(&[1, 2, 1212122, 1113]);
    bh.insert(-21);
    bh.insert(1);
    bh.insert(3);
    bh.insert(190);
    bh.insert(4);
    bh.insert(90);
    bh.extract();
    bh.extract();
    println!("{}", bh.peek().unwrap());
    println!("{}", bh.len());
}
