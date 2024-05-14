use es401::List2::List;

#[test]
fn list_pop_unique_element() {
  let mut l = List::<i32>::new();
  l.push(3);
  assert_eq!(l.pop(), Some(3))
}

#[test]
fn list_pop_from_empty_list_should_be_none() {
  let mut l = List::<i32>::new();
  assert_eq!(l.pop(), None)
}

#[test]
fn list_pop_multiple_element_should_return_in_reverse_order() {
  let mut l = List::<i32>::new();
  l.push(1);
  l.push(2);
  assert_eq!(l.pop(), Some(2));
  assert_eq!(l.pop(), Some(1));
}

#[test]
fn list_iter_should_return_iterator_in_reverse_order() {
  let mut l = List::<i32>::new();
  l.push(3);
  l.push(2);
  l.push(1);
  l.push(0);
  let mut i = 0;
  for e in l.iter() {
    assert_eq!(*e, i);
    i+=1;
  }
}

#[test]
fn list_take_should_split() {
  let mut l = List::<i32>::new();
  l.push(3);
  l.push(2);
  l.push(1);
  l.push(0);
  let mut l2 = l.take(2);
  assert_eq!(l2.pop(), Some(0));
  assert_eq!(l2.pop(), Some(1));
  assert_eq!(l.pop(), Some(2));
  assert_eq!(l.pop(), Some(3));
}
