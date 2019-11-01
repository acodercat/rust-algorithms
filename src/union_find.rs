use std::fmt::Debug;
use std::cmp::PartialOrd;
use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;
use std::fmt::Display;
use std::clone::Clone;

#[derive(Debug)]
pub struct UnionFind<T: Eq + Hash + Display + Clone + Debug> {
    parents: HashMap<T, T>,
    ranks: HashMap<T, usize>,
    len: usize,
    connected_component: usize
}

impl <T: Eq + Hash + Display + Clone + Debug> UnionFind <T> {

    pub fn new() -> Self {
        let parents: HashMap<T, T> = HashMap::new();
        let ranks: HashMap<T, usize> = HashMap::new();
        return UnionFind {
            parents,
            ranks,
            len: 0,
            connected_component: 0
        };
    }

    pub fn from(vec: Vec<T>) -> Self {
        let mut parents: HashMap<T, T> = HashMap::new();
        let mut ranks: HashMap<T, usize> = HashMap::new();
        let len = vec.len();
        for e in vec {
            ranks.insert(e.clone(), 1);
            parents.insert(e.clone(), e.clone());
        }

        return UnionFind {
            parents,
            ranks,
            len,
            connected_component: len
        };
    }

    pub fn union(&mut self, p: &T, q: &T) {
        let p_parent = self.find(p);
        let q_parent = self.find(q);
        if p_parent == None || q_parent == None {
            return;
        }

//        self.parents.insert(p_parent.unwrap().clone(), q_parent.unwrap().clone());
//        self.ranks.insert( q_parent.unwrap().clone(), 1);
//
//        if p_parent == q_parent {
//            return;
//        }
//
//        let p_rank = self.ranks.get(p_parent).unwrap();
//        let q_rank = self.ranks.get(q_parent).unwrap();

//        if p_rank > q_rank {
//            self.parents.insert(q_parent.clone(), p_parent.clone());
//        } else if p_rank < q_rank {
//            self.parents.insert(p_parent.clone(), q_parent.clone());
//        } else {
//            self.parents.insert(p_parent.clone(), q_parent.clone());
//            self.ranks.insert( q_parent.clone(), self.ranks.get(q_parent).unwrap() + 1);
//        }


//        println!("{:?}", self.parents.get(p_parent));
//        self.ranks.insert( q_parent.clone(), self.ranks.get(q_parent).unwrap() + 1);

        self.connected_component = self.connected_component + 1;
    }

    pub fn find(&self, element: &T) -> Option<&T> {
        let parent = self.parents.get(&element);
        return parent;
//        if  None == parent {
//            return None
//        }
//        if parent.unwrap() == element {
//            return parent;
//        }
//        return self.find(parent.unwrap());

        // pathCompression
//        self.parents.insert(e.clone(), self.parents.get(&(parent.unwrap())).unwrap().clone());
    }

    pub fn len(&self) -> usize {
        return self.len;
    }

    pub fn insert(&mut self, element: T) {
        self.parents.insert(element.clone(), element.clone());
    }

    pub fn connected_component(self) -> usize {
        return self.connected_component;
    }

//    pub fn is_connected(&mut self, q: T, p: T) -> bool {
//        return self.find(&q).unwrap() == self.find(&p).unwrap();
//    }
}