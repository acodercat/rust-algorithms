use algorithms::binary_heap::BinaryHeap;

fn main() {
    let mut heap:BinaryHeap<i32> = BinaryHeap::from(vec![1, 2, 3, 4]);
    heap.push(-21);
    heap.push(1);
    heap.push(3);
    heap.push(190);
    heap.push(4);
    heap.push(90);
    heap.extract();
    heap.extract();
    println!("{}", heap.peek().unwrap());
    println!("{}", heap.len());
}
