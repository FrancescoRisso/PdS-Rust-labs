use std::mem;

pub enum ListLink<T> {
    Cons(T, Box<ListLink<T>>),
    Nil,
}

pub struct List<T> {
    head: ListLink<T>,
}

impl<T> ListLink<T> {
    fn new() -> Self {
        ListLink::Nil
    }

    fn with_value(val: T, next: Box<ListLink<T>>) -> Self {
        ListLink::Cons(val, next)
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: ListLink::new(),
        }
    }

    // insert a new element at the beginning of the list
    // you may encouter a problem with the borrow checker while trying to move self.head to a new variable
    // why? look at mem::replace for solving it
    pub fn push(&mut self, elem: T) {
        let head = mem::replace(&mut self.head, ListLink::Nil);
        let new = ListLink::<T>::with_value(elem, Box::new(head));
        self.head = new;
    }

    pub fn pop(&mut self) -> Option<T> {
        let tmp = mem::replace(&mut self.head, ListLink::new());
        match tmp {
            ListLink::Nil => None,
            ListLink::Cons(val, next) => {
                self.head = *next;
                Some(val)
            }
        }
    }

    // return a referece to the first element of the list
    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            ListLink::Nil => None,
            ListLink::Cons(val, _) => Some(&val),
        }
    }

    // uncomment after having implemented the ListIter struct
    // return an interator over the list values
    pub fn iter(&self) -> ListIter<T> {
        ListIter::<T> { next: &self.head }
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
    next: &'a ListLink<T>,
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let a = Some(self.next);
        let b = &a;
        match b {
            &None => None,
            Some(ListLink::Nil) => None,
            Some(ListLink::Cons(val, next)) => {
                self.next = &*next;
                Some(&val)
            }
        }
    }
}

// something that may be useful for the iterator implementation:
// let a = Some(T);
// let b = &a;
// match b { Some(i) => ... } // here i is a reference to T
