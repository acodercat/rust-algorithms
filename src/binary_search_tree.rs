use std::fmt::Debug;
use std::cmp::PartialOrd;

#[derive(Debug)]
pub struct BinarySearchTree <T> {
    container: Vec<T>,
}