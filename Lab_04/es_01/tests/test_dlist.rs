use es_01::dlist::DList;

#[test]
fn list_push_head_pop_head_unique_element() {
    let mut l = DList::<i32>::new();
    l.push_head(3);
    assert_eq!(l.pop_head(), Some(3))
}

#[test]
fn list_push_tail_pop_tail_unique_element() {
    let mut l = DList::<i32>::new();
    l.push_tail(3);
    assert_eq!(l.pop_tail(), Some(3))
}

#[test]
fn list_push_head_pop_tail_unique_element() {
    let mut l = DList::<i32>::new();
    l.push_head(3);
    assert_eq!(l.pop_tail(), Some(3))
}

#[test]
fn list_push_tail_pop_head_unique_element() {
    let mut l = DList::<i32>::new();
    l.push_tail(3);
    assert_eq!(l.pop_head(), Some(3))
}

#[test]
fn list_pop_from_empty_list_should_be_none() {
    let mut l = DList::<i32>::new();
    assert_eq!(l.pop_head(), None);
    assert_eq!(l.pop_tail(), None);
}

#[test]
fn list_push_and_pop_head_multiple_element_should_return_in_reverse_order() {
    let mut l = DList::<i32>::new();
    l.push_head(1);
    l.push_head(2);
    assert_eq!(l.pop_head(), Some(2));
    assert_eq!(l.pop_head(), Some(1));
}

#[test]
fn list_push_and_pop_tail_multiple_element_should_return_in_reverse_order() {
    let mut l = DList::<i32>::new();
    l.push_tail(1);
    l.push_tail(2);
    assert_eq!(l.pop_tail(), Some(2));
    assert_eq!(l.pop_tail(), Some(1));
}

#[test]
fn list_push_head_and_pop_tail_multiple_element_should_return_in_order() {
    let mut l = DList::<i32>::new();
    l.push_head(1);
    l.push_head(2);
    assert_eq!(l.pop_tail(), Some(1));
    assert_eq!(l.pop_tail(), Some(2));
}

#[test]
fn list_push_tail_and_pop_head_multiple_element_should_return_in_order() {
    let mut l = DList::<i32>::new();
    l.push_tail(1);
    l.push_tail(2);
    assert_eq!(l.pop_head(), Some(1));
    assert_eq!(l.pop_head(), Some(2));
}

#[test]
fn list_iter_should_return_iterator_in_reverse_order() {
    let mut l = DList::<i32>::new();
    l.push_head(3);
    l.push_head(2);
    l.push_head(1);
    l.push_head(0);
    let mut i = 0;
    for e in l.iter() {
        assert_eq!(e, i);
        i += 1;
    }
}

#[test]
fn list_take_should_split() {
    let mut l = DList::<i32>::new();
    l.push_head(3);
    l.push_head(2);
    l.push_head(1);
    l.push_head(0);
    let mut l2 = l.take(2);
    assert_eq!(l2.pop_head(), Some(0));
    assert_eq!(l2.pop_head(), Some(1));
    assert_eq!(l.pop_head(), Some(2));
    assert_eq!(l.pop_head(), Some(3));
}
