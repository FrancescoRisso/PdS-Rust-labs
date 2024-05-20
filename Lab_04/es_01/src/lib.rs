pub mod dlist;
pub mod list1;
pub mod list2;

// *****
// double linked list suggestion: use Rc, since we need more than one reference to the same node
// for mutating the list and changing the next and prev fields we also need to be able to mutate the node, therefore we can use RefCell

// how to access content of Rc<RefCell<T>>:
// es let a = Rc::new(RefCell::new(5));
// let mut x = (*a).borrow_mut();  // with (*a) we dereference the Rc, with (*a).borrow_mut() we get a mutable reference to the content of the RefCell
// *x = 6; // we can now change the content of the RefCell

// to take a value from a Rc (useful when popping a value from the list): usually it is not possible since it may be referenced elsewhere.
// if you can guarantee it's the only reference to the value  youu can use Rc::try_unwrap(a).unwrap().into_inner() to get the value
// it first takes out the value from the Rc, then it tries to unwrap the value from the Result, and finally it takes the inner value from the Result
// see here
// https://stackoverflow.com/questions/70404603/how-to-return-the-contents-of-an-rc

// other hint that may be useful: Option<T> has a default clone implementation which calls the clone of T. Therefore:
// Some(T).clone() ->  Some(T.clone())
// None.clone() -> None

//  type NodeLink = Option<Rc<RefCell<DNode>>>; // we define a type alias for better readibility
// Example
//  type NodeBackLink = ...

// struct DNode {
// v: i32,
// prev: NodeBackLink // here we can't put NodeLink to avoid a cycle reference, what do we use?
// next: NodeLink
// }

// struct DList {
// head: NodeLink,
// tail: NodeLink
// }
