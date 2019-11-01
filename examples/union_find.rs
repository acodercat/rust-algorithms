use algorithms::union_find::UnionFind;
//use core::ptr::NonNull;
//use std::ptr::read;
use std::collections::HashMap;
use std::rc::Rc;

//struct Node<'a, T> {
////    next: Option<NonNull<Node<T>>>,
////    prev: Option<NonNull<Node<T>>>,
//    str: &'a String,
//    element: T,
//}
//
//fn test <'a> (mut s1: &'a String, mut s2: &'a String) -> &'a String {
//   if s1.len() > s2.len(){
//       return s1;
//   }
//    return s2;
//}

//static mut C: i32 = 10;
fn main() {
//    let mut map = HashMap::new();
//    map.insert("a", 1);
//    map.insert("a", 2);
////    map.insert("a", "b");
////    map.insert("a1", "b");
////    map.insert("a2", "b");
//    for (k,v) in map {
//        println!("{}", v);
//    }

    let x = Rc::new(45);
    let y1 = x.clone();
    let y2 = x.clone();

    println!("{}", Rc::strong_count(&x));



//    let mut union_find = UnionFind::new();
//    let str = 1;
//    union_find.insert(str);

//    let mut s = String::from("1212");
////    let b = s;
//    let s1 = &mut s;
////    let s2 = &mut s;
//    let s2 = &s;
//    println!("{}, {}", s1, s2);


//    println!("{}", str);
//    let s1 = String::from("adsad");
//    let s2 = String::from("1212");
//    let s3:&String;
//    {
//        s3 = test(&s1, &s2);
//    }
//
//    println!("{}", s3);

//    let mut node = Node {
//        element: 1
//    };
//    node.element = 2;
//    let mut a = 1;
//    let p:*mut i32 = &mut a as *mut i32;
//    unsafe {
//        p.write(1);
//    }
//    let *mut a = &String::from("1212");
//    let mut b = String::from("hello");
////    let b = &a;
////    let c = & a;
//    let mut d = &mut a;
//    d =  &mut b;
//    println!("{}", a);
//    println!("{}", *d)
//    let mut r = 1;
//    test(&r);
//    test
//    let mut x = 111;
//    r = &x;
//    {
//        let x = 5;
//        r = &x;
//    }

}
