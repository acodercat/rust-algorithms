#[cfg(test)]
mod tests {
    use algorithms::tree::BinaryHeap;

    #[test]
    fn test_binary_heap() {
        let mut bh1:BinaryHeap<i32> = BinaryHeap::from_array(&[1,3,4,5]);
        bh1.insert(-21);
        bh1.insert(1);
        bh1.insert(3);
        bh1.insert(190);
        assert_eq!(*bh1.peek().unwrap(), 190);
        assert_eq!(bh1.extract().unwrap(), 190);
        assert_eq!(*bh1.peek().unwrap(), 5);

        let mut bh2:BinaryHeap<i32> = BinaryHeap::new();
        bh2.insert(-21);
        bh2.insert(1);
        bh2.insert(3);
        bh2.insert(190);
        assert_eq!(*bh2.peek().unwrap(), 190);
        assert_eq!(bh2.extract().unwrap(), 190);
        assert_eq!(*bh2.peek().unwrap(), 3);
    }

}
