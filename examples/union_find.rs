use algorithms::UnionFind;

fn main() {

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


