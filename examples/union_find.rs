use algorithms::union_find::UnionFind;
//use std::collections::HashMap;

fn main() {

    let mut union_find:UnionFind = UnionFind::new();
    union_find.push(2);
    union_find.push(21);
    union_find.push(3);
    union_find.push(1);
    union_find.push(9);
    union_find.union(3,1);
    union_find.union(3,2);
    union_find.union(3,21);
    println!("{:?}", union_find.connected_component());
//    let mut hm = HashMap::new();
//    hm.insert(1, 2);
//    hm.insert(11, 21);
//    println!("{:?}", hm.contains_key(&111));


}


