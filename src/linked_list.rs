use std::ptr::NonNull;

struct Node <T> {
    element: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>
}

impl <T> Node <T> {
    pub fn new (element: T) -> Self {
        Self {
            element,
            next: None,
            prev: None
        }
    }
}

pub struct LinkedList <T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

impl<T> LinkedList <T> {

    pub fn new() -> Self {
        Self {
            len: 0,
            head: None,
            tail: None
        }
    }

    #[inline]
    pub fn push_front(&mut self, element: T) {
        let mut new_node = Node::new(element);
        new_node.next = self.head;
        let new_node = Some(Box::into_raw_non_null(Box::new(new_node)));
        if let None = self.tail {
            self.tail = new_node;
        }

        if let Some(mut head) = self.head {
            unsafe {
                head.as_mut().prev = new_node
            }
        }

        self.head = new_node;
        self.len += 1;
    }

    #[inline]
    pub fn pop_front(&mut self) -> Option<T> {

        self.head.map(|head| unsafe {
            let head = Box::from_raw(head.as_ptr());

            self.head = head.next;
            match self.head {
                Some(mut new_head) => new_head.as_mut().prev = None,
                None => self.tail = None
            }
            self.len -= 1;
            head.element
        })
    }

    #[inline]
    pub fn push_back(&mut self, element: T) {
        let mut new_node = Node::new(element);
        new_node.prev = self.tail;
        let new_node = Some(Box::into_raw_non_null(Box::new(new_node)));
        if let None = self.head {
            self.head = new_node
        };

        if let Some(mut tail) = self.tail {
            unsafe { tail.as_mut().next = new_node };
        };

        self.tail = new_node;
        self.len += 1;
    }

    #[inline]
    pub fn pop_back(&mut self) -> Option<T> {

        self.tail.map(|tail| unsafe {
            let tail = Box::from_raw(tail.as_ptr());
            self.tail = tail.prev;

            match self.tail {
                None => self.head = None,
                Some(mut new_tail) => new_tail.as_mut().next = None
            }

            self.len -= 1;
            tail.element
        })
    }

}