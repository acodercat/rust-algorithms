use std::fmt::Debug;
use std::collections::HashMap;

#[derive(Debug)]
pub struct UnionFind {
    parents: HashMap<usize, usize>,
    ranks: HashMap<usize, usize>,
    len: usize,
    connected_component: usize
}

impl UnionFind {

    pub fn new() -> Self {
        let parents = HashMap::<usize, usize>::new();
        let ranks = HashMap::<usize, usize>::new();
        return UnionFind {
            parents,
            ranks,
            len: 0,
            connected_component: 0
        };
    }

    pub fn from(vec: Vec<usize>) -> Self {
        let mut parents = HashMap::<usize, usize>::new();
        let mut ranks = HashMap::<usize, usize>::new();
        let len = vec.len();
        for element in vec {
            ranks.insert(element, 1);
            parents.insert(element, element);
        }

        return UnionFind {
            parents,
            ranks,
            len,
            connected_component: len
        };
    }

    /// Union two elements
    ///
    /// # Examples
    ///
    /// ```
    /// use algorithms::UnionFind;
    ///
    /// let mut union_find: UnionFind = UnionFind::new();
    /// union_find.insert(8);
    /// union_find.insert(18);
    /// union_find.union(8,18);
    /// assert_eq!(union_find.is_connected(8, 18), true);
    /// ```
    #[inline]
    pub fn union(&mut self, p: usize, q: usize) -> bool {
        if let (Some(p_parent), Some(q_parent)) = (self.find(p), self.find(q)) {
            if p_parent != q_parent {
                if let (Some(p_rank), Some(q_rank)) = (self.ranks.get(&p_parent), self.ranks.get(&q_parent)) {
                    if p_rank > q_rank {
                        self.parents.insert(q_parent, p_parent);
                    } else if p_rank < q_rank {
                        self.parents.insert(p_parent, q_parent);
                    } else {
                        self.ranks.insert( q_parent, self.ranks.get(&(q_parent)).unwrap() + 1);
                        self.parents.insert(p_parent, q_parent);
                    }
                    self.connected_component -= 1;
                    return true;
                }
            }
        };
        return false;
    }

    /// Returns the length of the `UnionFind`.
    ///
    /// # Examples
    ///
    /// ```
    /// use algorithms::UnionFind;
    ///
    /// let mut union_find: UnionFind = UnionFind::new();
    ///
    /// uf.insert(2);
    /// assert_eq!(uf.len(), 1);
    ///
    /// uf.insert(21);
    /// assert_eq!(uf.len(), 2);
    ///
    /// uf.insert(23);
    /// assert_eq!(uf.len(), 3);
    /// ```
    #[inline]
    pub fn find(&self, element: usize) -> Option<usize> {
        let mut current_element= element;
        while let Some(parent) = self.parents.get(&current_element) {
            if let Some(parent_of_parent) = self.parents.get(parent) {
                if parent == parent_of_parent {
                    return Some(*parent_of_parent);
                }
                current_element = *parent_of_parent;
            };
        };
        None
    }

    /// Returns the length of the `UnionFind`.
    ///
    /// # Examples
    ///
    /// ```
    /// use algorithms::UnionFind;
    ///
    /// let mut union_find: UnionFind = UnionFind::new();
    ///
    /// union_find.insert(2);
    /// assert_eq!(union_find.len(), 1);
    ///
    /// union_find.insert(21);
    /// assert_eq!(union_find.len(), 2);
    ///
    /// union_find.insert(23);
    /// assert_eq!(union_find.len(), 3);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        return self.len;
    }

    /// Returns the length of the `UnionFind`.
    ///
    /// # Examples
    ///
    /// ```
    /// use algorithms::UnionFind;
    ///
    /// let mut union_find: UnionFind = UnionFind::new();
    ///
    /// union_find.insert(2);
    /// assert_eq!(union_find.len(), 1);
    ///
    /// union_find.insert(21);
    /// assert_eq!(union_find.len(), 2);
    ///
    /// union_find.insert(23);
    /// assert_eq!(union_find.len(), 3);
    /// ```
    #[inline]
    pub fn insert(&mut self, element: usize) {
        if !self.parents.contains_key(&element) {
            self.parents.insert(element, element);
            self.ranks.insert(element, 1);
            self.connected_component += 1;
        }
    }

    ///  Returns the connected component of the `UnionFind`.
    ///
    /// # Examples
    ///
    /// ```
    /// use algorithms::UnionFind;
    ///
    /// let mut union_find: UnionFind = UnionFind::new();
    ///
    /// union_find.insert(2);
    /// union_find.insert(21);
    /// assert_eq!(union_find.connected_component(), 2);
    ///
    /// union_find.union(2, 21);
    /// assert_eq!(union_find.connected_component(), 1);
    /// ```
    #[inline]
    pub fn connected_component(&self) -> usize {
        return self.connected_component;
    }

    /// Returns `true` if the two elements is connected.
    ///
    /// # Examples
    ///
    /// ```
    /// use algorithms::UnionFind;
    ///
    /// let mut union_find: UnionFind = UnionFind::new();
    /// union_find.insert(8);
    /// union_find.insert(18);
    /// union_find.union(8, 18);
    /// assert_eq!(union_find.is_connected(8, 18), true);
    /// ```
    #[inline]
    pub fn is_connected(&self, q: usize, p: usize) -> bool {
        if let (Some(p_parent), Some(q_parent)) = (self.find(p), self.find(q)) {
            if p_parent == q_parent {
                return true;
            }
        }
        return false;
    }
}

#[test]
fn test_union_find() {
    let mut union_find1: UnionFind = UnionFind::new();
    union_find1.insert(2);
    union_find1.insert(21);
    union_find1.insert(3);
    assert_eq!(union_find1.connected_component(), 3);
    assert_eq!(union_find1.is_connected(2, 3), false);
    assert_eq!(union_find1.union(3,2), true);
    assert_eq!(union_find1.union(3,2), false);
    assert_eq!(union_find1.is_connected(2, 3), true);
    assert_eq!(union_find1.connected_component(), 2);

    let mut union_find2: UnionFind = UnionFind::from(vec![1, 2, 3, 4]);
    assert_eq!(union_find2.connected_component(), 4);
    union_find2.insert(11);
    assert_eq!(union_find2.connected_component(), 5);
    assert_eq!(union_find2.is_connected(1, 2), false);
    assert_eq!(union_find2.union(3,2), true);
    assert_eq!(union_find2.union(3,2), false);
    assert_eq!(union_find2.is_connected(2, 3), true);
    assert_eq!(union_find2.connected_component(), 4);
}