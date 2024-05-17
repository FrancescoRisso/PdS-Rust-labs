use std::mem;

pub struct Node<T> {
    elem: T,
    next: NodeLink<T>,
}

type NodeLink<T> = Option<Box<Node<T>>>;

pub struct List<T>
// where
//     T: Deref,
{
    head: NodeLink<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Node { elem, next: None }
    }

    fn with_next(elem: T, next: Box<Node<T>>) -> Self {
        Node {
            elem,
            next: Some(next),
        }
    }
}

// for this implementattion, since we are using option, take a look at the take method in Option<T>.
// It allows to move the value of the option into another option and replace it with None
// let mut a = Some(5);
// let b = a.take(); // a is now None and b is Some(5)
impl<T> List<T> {
    // impl<T> List<T> where T: Deref {
    pub fn new() -> Self {
        List { head: None }
    }

    // insert a new element at the beginning of the list
    // you may encouter a problem with the borrow checker while trying to move self.head to a new variable
    // why? look at mem::replace for solving it
    pub fn push(&mut self, elem: T) {
        if self.head.is_none() {
            self.head = Some(Box::new(Node::new(elem)));
            return;
        }

        let head = mem::replace(&mut self.head, None).unwrap();
        let new_node = Node::with_next(elem, head);
        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                let _ = mem::replace(&mut self.head, node.next);
                Some(node.elem)
            }
        }
    }

    // return a referece to the first element of the list
    pub fn peek<'a>(&'a self) -> Option<&'a T> {
        match &self.head {
            None => None,
            Some(node) => Some(&node.elem),
        }
    }

    // uncomment after having implemented the ListIter struct
    // return an interator over the list values
    pub fn iter(&self) -> ListIter<T> {
        match &self.head {
            None => ListIter { next: None },
            Some(node) => ListIter { next: Some(node) },
        }
    }

    pub fn reverse(mut self) -> Self {
        let mut tmp = List::new();

        loop {
            match self.pop() {
                None => break,
                Some(val) => tmp.push(val),
            }
        }

        tmp
    }

    // take the first n elements of the list and return a new list with them
    pub fn take(&mut self, n: usize) -> List<T> {
        let mut tmp = List::new();

        for _ in 0..n {
            match self.pop() {
                None => break,
                Some(val) => tmp.push(val),
            }
        }

        tmp.reverse()
    }
}

pub struct ListIter<'a, T> {
    next: Option<&'a Box<Node<T>>>,
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.next {
            &None => None,
            &Some(node) => {
                match &node.next {
                    None => self.next = None,
                    Some(next) => self.next = Some(&next),
                };
                Some(&node.elem)
            }
        }
    }
}
