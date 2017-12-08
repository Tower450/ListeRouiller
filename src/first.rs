// in first.rs
use std::mem;

// pub says we want people outside this module to be able to use List
pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
         List { head: Link::Empty }    
    }

    pub fn push(&mut self, elem: i32){
        let n = Box::new(Node {
            elem: elem,
            //self.head deviens empty et next deviens l'ancien head avant d'etre empty.
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(n)
    }


    // Check if the list is empty.
    // If it's empty, just return None
    // It it's not empty
    // remove the head of the list
    // remove its elem
    // replace the lists head with its next
    // return Some(elem)
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(boxed_node) => {
                let node = *boxed_node;
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}


mod test {
    #[test]
    fn basics() {
        let mut list = super::List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}