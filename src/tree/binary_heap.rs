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
    pub fn from_array(arr: &[T]) -> Self where T: Clone {
        let mut bh = BinaryHeap {
            container: arr.to_vec()
        };
        for i in (0..=bh.calculate_parent_index_of_tail()).rev() {
            bh.shift_down(i);
        }
        return bh;
    }

    pub fn insert(&mut self, element: T) {
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

    fn shift_down(&mut self, index: usize) {
        let mut current_index = index;
        let mut max_child_index;
        let mut left_child_index_of_current = Self::calculate_left_child_index(current_index);
        while left_child_index_of_current < self.len() {
            let right_child_index_of_current = Self::calculate_right_child_index(current_index);
            if right_child_index_of_current < self.len() && self.container.get(right_child_index_of_current) > self.container.get(left_child_index_of_current) {
                max_child_index = right_child_index_of_current;
            } else {
                max_child_index = left_child_index_of_current;
            }

            if self.container.get(current_index) >= self.container.get(max_child_index) {
                break;
            }

            self.container.swap(current_index, max_child_index);
            current_index = max_child_index;
            left_child_index_of_current = Self::calculate_left_child_index(current_index);
        }
    }

    fn shift_up(&mut self, index: usize) {
        let mut current_index = index;
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