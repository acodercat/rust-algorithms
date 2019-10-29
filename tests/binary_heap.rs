#[cfg(test)]
mod tests {
    use algorithms::tree::BinaryHeap;

    #[test]
    fn test_binary_heap() {
        let mut heap1:BinaryHeap<i32> = BinaryHeap::from(vec![1,3,4,5]);
        heap1.insert(-21);
        heap1.insert(1);
        heap1.insert(3);
        heap1.insert(190);
        assert_eq!(heap1.peek(), Some(&190));
        assert_eq!(heap1.extract(), Some(190));
        assert_eq!(heap1.peek(), Some(&5));

        let mut heap2:BinaryHeap<i32> = BinaryHeap::new();
        heap2.insert(-21);
        heap2.insert(1);
        heap2.insert(3);
        heap2.insert(190);
        assert_eq!(heap2.peek(), Some(&190));
        assert_eq!(heap2.extract(), Some(190));
        assert_eq!(heap2.peek(), Some(&3));
    }

}
