#![feature(box_into_raw_non_null)]
#![feature(box_syntax)]

use std::ptr::NonNull;
use std::marker::PhantomData;
use std::ptr::null;
use std::mem;
use algorithms::LinkedList;
use std::thread;

#[derive(Debug)]
struct User {
    age: i32
}



#[derive(Debug)]
enum Foo {
    f(i32),
    o(i32),
    o1(i32)
}


static USER: User = User {
    age: 1
};

//fn test1(mut u: &User) {
//    u = & User {
//        age: 1
//    };
//}
//
//fn test2(mut u: &User) {
//    let t = User {
//        age: 1
//    };
//    u = & t;
//}



fn capitalize(data: &mut [char]) {
    for c in data {
        c.make_ascii_lowercase();
    }
}


fn main() {

    let mut data = vec!['a', 'b', 'c'];
    let slice = &mut data[..];
    capitalize(slice);
    data.push('d');
    data.push('d');
    data.push('d');
    data.push('d');












    let mut u = & User {
        age: 2
    };
//    test(&mut u);
//    test2(&USER);
    println!("s: {:?}", u);


    let mut data = vec!['a', 'b', 'c'];
    let a = &mut data;

    a[0] = 'b';
    println!("a: {:?}", a);
    println!("data: {:?}", data);

    let u = &mut User {
        age: 2
    };

    u.age = 3;


//    let f = Foo::f(1);

    println!("size_of: {:?}", std::mem::size_of::<Foo>());



//    let mut u: & User = & User {
//        age: 2
//    };
//    test(&mut u);
//    println!("s: {:?}", u);


    let mut a = 31;
    let b = box a;
    println!("a: {:?}", a);

    println!("b: {:?}", b);
//    unsafe { test() };
//    let mut linked_list = LinkedList::new();
//    linked_list.push_front(1);
//    linked_list.push_front(2);
//    linked_list.push_front(3);
//    linked_list.push_front(8);
//    linked_list.push_back(18);
//    let mut node = linked_list.pop_back();
//    node = linked_list.pop_back();
//
//    node = linked_list.pop_back();
//    node = linked_list.pop_back();
//    node.map(|node| {
//        println!("node: {:?}", node);
//    });
//    node = linked_list.pop_back();
//    node = linked_list.pop_back();
//    node = linked_list.pop_back();
//    node = linked_list.pop_back();

    let a;
    {
        let mut b = 12;
        a = &mut b as *mut i32;
    }


    let s = String::from("12121");

    println!("a: {:?}", unsafe {a.read()});


}