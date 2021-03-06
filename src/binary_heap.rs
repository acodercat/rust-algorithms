use std::fmt::Debug;
use std::cmp::PartialOrd;

#[derive(Debug)]
pub struct BinaryHeap <T> {
    container: Vec<T>,
}

impl <T: Debug + PartialOrd> BinaryHeap <T> {

    pub fn new() -> Self {
        return BinaryHeap {
            container: Vec::new()
        };
    }

    // heapify
    pub fn from(vec: Vec<T>) -> BinaryHeap<T> {
        let mut heap = BinaryHeap {
            container: vec
        };
        for i in (0 ..= heap.calculate_parent_index_of_tail()).rev() {
            heap.shift_down(i);
        }
        return heap;
    }

    pub fn push(&mut self, element: T) {
        self.container.push(element);
        self.shift_up(self.tail_index());
    }

    pub fn len(&self)-> usize {
        return self.container.len();
    }

    pub fn tail_index(&self)-> usize {
        return self.len() - 1;
    }

    pub fn capacity(self) -> usize {
        return self.container.capacity();
    }

    pub fn is_empty(self) -> bool {
        return self.container.is_empty();
    }

    pub fn peek(&self) -> Option<&T> {
        return self.container.first();
    }

    pub fn extract(&mut self) -> Option<T> {
        let tail_index = self.tail_index();
        let head_index = 0;
        self.container.swap(head_index, tail_index);
        let root= self.container.pop();
        self.shift_down(head_index);
        return root;
    }

    fn shift_down(&mut self, mut current_index: usize) {
        let mut found_child_index;
        let mut left_child_index_of_current = Self::calculate_left_child_index(current_index);
        while left_child_index_of_current < self.len() {
            let right_child_index_of_current = Self::calculate_right_child_index(current_index);
            if right_child_index_of_current < self.len() && self.container.get(right_child_index_of_current) > self.container.get(left_child_index_of_current) {
                found_child_index = right_child_index_of_current;
            } else {
                found_child_index = left_child_index_of_current;
            }

            if self.container.get(current_index) >= self.container.get(found_child_index) {
                break;
            }

            self.container.swap(current_index, found_child_index);
            current_index = found_child_index;
            left_child_index_of_current = Self::calculate_left_child_index(current_index);
        }
    }

    fn shift_up(&mut self, mut current_index: usize) {
        let mut parent_index_of_current = Self::calculate_parent_index(current_index);
        while (current_index > 0) && (self.container.get(current_index) > self.container.get(parent_index_of_current)) {
            self.container.swap(current_index, parent_index_of_current);
            current_index = parent_index_of_current;
            parent_index_of_current = Self::calculate_parent_index(current_index);
        }
    }


    fn calculate_left_child_index(index: usize) -> usize {
        return index * 2 + 1;
    }

    fn calculate_right_child_index(index: usize) -> usize {
        return Self::calculate_left_child_index(index) + 1;
    }

    fn calculate_parent_index(index: usize) -> usize {
        let parent_index = ((index as f32 - 1.0) / 2.0).floor() as usize;
        if parent_index <= 0 {
            return 0;
        }
        return parent_index;
    }

    fn calculate_parent_index_of_tail(&self) -> usize {
        let tail_index = self.len() - 1;
        return Self::calculate_parent_index(tail_index);
    }
}

#[test]
fn test_binary_heap() {
    let mut heap1:BinaryHeap<i32> = BinaryHeap::from(vec![1,3,4,5]);
    heap1.push(-21);
    heap1.push(1);
    heap1.push(3);
    heap1.push(190);
    assert_eq!(heap1.peek(), Some(&190));
    assert_eq!(heap1.extract(), Some(190));
    assert_eq!(heap1.peek(), Some(&5));

    let mut heap2:BinaryHeap<i32> = BinaryHeap::new();
    heap2.push(-21);
    heap2.push(1);
    heap2.push(3);
    heap2.push(190);
    assert_eq!(heap2.peek(), Some(&190));
    assert_eq!(heap2.extract(), Some(190));
    assert_eq!(heap2.peek(), Some(&3));
}