use algorithms::tree::BinaryHeap;

fn main() {
    let mut heap:BinaryHeap<i32> = BinaryHeap::from(vec![1, 2, 3, 4]);
    heap.insert(-21);
    heap.insert(1);
    heap.insert(3);
    heap.insert(190);
    heap.insert(4);
    heap.insert(90);
    heap.extract();
    heap.extract();
    println!("{}", heap.peek().unwrap());
    println!("{}", heap.len());
}
