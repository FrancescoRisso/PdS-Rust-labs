use std::{
    cell::{Ref, RefCell},
    rc::{Rc, Weak},
};

pub struct DNode<T> {
    elem: T,
    next: NextLink<T>,
    prev: PrevLink<T>,
}

type NextLink<T> = Option<Rc<RefCell<DNode<T>>>>;
type PrevLink<T> = Option<Weak<RefCell<DNode<T>>>>;

pub struct DList<T> {
    head: NextLink<T>,
    tail: PrevLink<T>,
}

impl<T> DNode<T> {
    fn new(elem: T, next: NextLink<T>, prev: PrevLink<T>) -> Self {
        DNode { elem, next, prev }
    }

    fn set_next(&mut self, next: Rc<RefCell<DNode<T>>>) {
        self.next = Some(next);
    }

    fn set_next_none(&mut self) {
        self.next = None;
    }

    fn set_prev(&mut self, prev: Rc<RefCell<DNode<T>>>) {
        let weak = Rc::downgrade(&prev);
        self.prev = Some(weak);
    }

    fn set_prev_none(&mut self) {
        self.prev = None;
    }
}

// for this implementattion, since we are using option, take a look at the take method in Option<T>.
// It allows to move the value of the option into another option and replace it with None
// let mut a = Some(5);
// let b = a.take(); // a is now None and b is Some(5)
impl<T> DList<T> {
    // impl<T> List<T> where T: Deref {
    pub fn new() -> Self {
        DList {
            head: None,
            tail: None,
        }
    }

    fn set_head_none(&mut self) {
        self.head = None;
    }

    fn set_head(&mut self, next: Rc<RefCell<DNode<T>>>) {
        self.head = Some(next);
    }

    fn set_tail_none(&mut self) {
        self.tail = None;
    }

    fn set_tail(&mut self, prev: Rc<RefCell<DNode<T>>>) {
        let weak = Rc::downgrade(&prev);
        self.tail = Some(weak);
    }

    // insert a new element at the beginning of the list
    // you may encouter a problem with the borrow checker while trying to move self.head to a new variable
    // why? look at mem::replace for solving it
    pub fn push_head(&mut self, elem: T) {
        match &self.head {
            None => {
                let node = RefCell::new(DNode::new(elem, None, None));
                let rc = Rc::from(node);
                self.set_head(rc.clone());
                self.set_tail(rc);
            }
            Some(head) => {
                let node = RefCell::new(DNode::new(elem, Some(head.clone()), None));
                let rc = Rc::new(node);
                head.as_ref().borrow_mut().set_prev(rc.clone());
                self.set_head(rc);
            }
        }
    }

    pub fn pop_head(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        let head = self.head.take().unwrap();
        // let next = &head.as_ref().borrow().next;

        match &head.as_ref().borrow().next {
            None => self.set_tail_none(),
            Some(rc_next) => {
                rc_next.clone().as_ref().borrow_mut().set_prev_none();
                self.set_head(Rc::clone(rc_next));
            }
        }

        match Rc::try_unwrap(head) {
            Err(e) => {
                _ = e;
                None
            }
            Ok(head_refcell) => Some(head_refcell.into_inner().elem),
        }
    }

    pub fn push_tail(&mut self, elem: T) {
        match &self.tail {
            None => {
                let node = RefCell::new(DNode::new(elem, None, None));
                let rc = Rc::from(node);
                self.set_head(rc.clone());
                self.set_tail(rc);
            }
            Some(tail) => {
                let node = RefCell::new(DNode::new(elem, None, Some(tail.clone())));
                let rc = Rc::new(node);
                match Weak::upgrade(tail) {
                    None => {}
                    Some(tail_ptr) => tail_ptr.as_ref().borrow_mut().set_next(rc.clone()),
                }
                self.set_tail(rc);
            }
        }
    }

    pub fn pop_tail(&mut self) -> Option<T> {
        if self.tail.is_none() {
            return None;
        }

        let tail_ptr = self.tail.take().unwrap();
        // let next = &head.as_ref().borrow().next;

        match tail_ptr.upgrade() {
            None => return None,
            Some(tail) => {
                match &tail.as_ref().borrow().prev {
                    None => self.set_head_none(),
                    Some(weak_prev) => match weak_prev.upgrade() {
                        None => return None,
                        Some(rc_prev) => {
                            rc_prev.clone().as_ref().borrow_mut().set_next_none();
                            self.set_tail(Rc::clone(&rc_prev));
                        }
                    },
                };

                match Rc::try_unwrap(tail) {
                    Err(_) => None,
                    Ok(tail_refcell) => Some(tail_refcell.into_inner().elem),
                }
            }
        }
    }

    // return a referece to the first element of the list
    pub fn peek_head<'a>(&'a self) -> Option<Ref<T>> {
        match &self.head {
            None => None,
            Some(rc) => Some(Ref::map(rc.as_ref().borrow(), |node| &node.elem)),
        }
    }

    // uncomment after having implemented the ListIter struct
    // return an interator over the list values
    pub fn iter(&self) -> ListIter<T> {
        match &self.head {
            None => ListIter { next: None },
            Some(node) => ListIter {
                next: Some(node.clone()),
            },
        }
    }

    pub fn reverse(mut self) -> Self {
        let mut tmp = DList::new();

        loop {
            match self.pop_head() {
                None => break,
                Some(val) => tmp.push_head(val),
            }
        }

        tmp
    }

    // take the first n elements of the list and return a new list with them
    pub fn take(&mut self, n: usize) -> DList<T> {
        let mut tmp = DList::new();

        for _ in 0..n {
            match self.pop_head() {
                None => break,
                Some(val) => tmp.push_head(val),
            }
        }

        tmp.reverse()
    }
}

pub struct ListIter<T> {
    next: NextLink<T>,
}

impl<T> Iterator for ListIter<T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let val = {
            let this_node = self.next.as_ref()?.as_ref().borrow_mut();
            this_node.elem.clone()
        };

        let next_ptr = {
            let this_node = self.next.as_ref()?.as_ref().borrow_mut();
            this_node.next.clone()
        };

        self.next = next_ptr;
        Some(val)
    }
}
