use std::{thread, time};

use es_02::{CircularBuffer, TSCircularBuffer};

#[test]
fn insert_element_should_increment_buffer_size() {
    let mut b = CircularBuffer::new(10);
    let _ = b.write(3);
    assert_eq!(b.size(), 1);
}

#[test]
fn read_element_should_return_inserted_element() {
    let item = 4;
    let mut b = CircularBuffer::new(10);
    let _ = b.write(item);
    let read = b.read().unwrap();
    assert_eq!(read, item);
}

#[test]
fn read_multiple_element_should_return_inserted_element_in_order() {
    let i1 = 1;
    let i2 = 2;
    let i3 = 3;
    let mut b = CircularBuffer::new(10);
    let _ = b.write(i1);
    let _ = b.write(i2);
    let _ = b.write(i3);
    let r1 = b.read().unwrap();
    let r2 = b.read().unwrap();
    let r3 = b.read().unwrap();
    assert_eq!((i1, i2, i3), (r1, r2, r3));
}

#[test]
fn buffer_is_circular() {
    let mut b = CircularBuffer::new(2);
    let i = [1, 2, 3];
    let mut r = vec![0; 3];
    let _ = b.write(i[0]);
    let _ = b.write(i[1]);
    r[0] = b.read().unwrap();
    let _ = b.write(i[2]);
    r[1] = b.read().unwrap();
    r[2] = b.read().unwrap();
    assert_eq!(r, i);
}

#[test]
fn read_from_empty_buffer_should_return_none() {
    let mut b: CircularBuffer<i32> = CircularBuffer::new(10);
    assert_eq!(b.read(), None);
}

#[test]
fn write_on_full_buffer_should_return_err() {
    let mut b = CircularBuffer::new(1);
    let _ = b.write(1);
    let res = match b.write(2) {
        Ok(_) => true,
        Err(_) => false,
    };
    assert_eq!(res, false);
}

#[test]
fn overwrite_should_act_as_write_if_not_full() {
    let mut b = CircularBuffer::new(2);
    b.overwrite(1);
    assert_eq!(b.read().unwrap(), 1);
}

#[test]
fn overwrite_should_replace_first_element_if_buffer_full() {
    let mut b = CircularBuffer::new(2);
    let i = [1, 2, 3];
    let mut r = [0, 0];
    let _ = b.write(i[0]);
    let _ = b.write(i[1]);
    b.overwrite(i[2]);
    r[0] = b.read().unwrap();
    r[1] = b.read().unwrap();
    assert_eq!(r, i[1..=2]);
}

#[test]
fn make_contigous_should_put_head_before_tail() {
    let mut b = CircularBuffer::new(3);
    let i = [1, 2, 3, 4];
    let _ = b.write(i[0]);
    let _ = b.write(i[1]);
    let _ = b.write(i[2]);
    let _ = b.read();
    let _ = b.read();
    let _ = b.write(i[3]);
    b.make_contiguous();
    assert_eq!((b.get_head(), b.get_tail()), (0, 2));
}

// threads

#[test]
fn ts_read_after_write_should_return_the_value() {
    let b = TSCircularBuffer::new(CircularBuffer::new(3));
    let item = 2;
    let _ = b.lock().unwrap().write(item);
    assert_eq!(b.lock().unwrap().read().unwrap(), item);
}

#[test]
fn ts_avg_full() {
    let b = TSCircularBuffer::new(CircularBuffer::new(3));

    thread::scope(|s| {
        s.spawn(|| {
            for i in 0..10 {
                let _ = b.lock().unwrap().write(i);
                println!("W{:3}", i);
                std::thread::sleep(time::Duration::from_secs(1));
            }
        });

        s.spawn(|| {
            let mut cnt = 0;
            for _ in 0..5 {
                match b.lock().unwrap().read() {
                    Some(el) => {
                        println!("      R{:3}", el);
                        assert_eq!(el, cnt);
                        cnt += 1;
                    }
                    None => println!("      R___"),
                }
                std::thread::sleep(time::Duration::from_secs(2));
            }
        });
    });
}

#[test]
fn ts_avg_empty() {
    let b = TSCircularBuffer::new(CircularBuffer::new(3));

    thread::scope(|s| {
        s.spawn(|| {
            for i in 0..5 {
                let _ = b.lock().unwrap().write(i);
                println!("W{:3}", i);
                std::thread::sleep(time::Duration::from_secs(2));
            }
        });

        s.spawn(|| {
            let mut cnt = 0;
            for _ in 0..5 {
                match b.lock().unwrap().read() {
                    Some(el) => {
                        println!("      R{:3}", el);
                        assert_eq!(el, cnt);
                        cnt += 1;
                    }
                    None => println!("      R___"),
                }
                std::thread::sleep(time::Duration::from_secs(1));
            }
        });
    });
}

#[test]
fn ts_throug() {
    let b = TSCircularBuffer::new(CircularBuffer::new(1000));

    thread::scope(|s| {
        s.spawn(|| {
            for i in 0..100000 {
                let _ = b.lock().unwrap().write(i);
                //println!("W{:3}", i);
                std::thread::sleep(time::Duration::from_nanos(1));
            }
        });

        s.spawn(|| {
            let mut cnt = 0;
            let start = time::Instant::now();
            for _ in 0..100000 {
                match b.lock().unwrap().read() {
                    Some(_) => {
                        //println!("      R{:3}", el);
                        cnt += 1;
                    }
                    None => (), //println!("      R___"),
                }
                std::thread::sleep(time::Duration::from_nanos(1));
            }
            println!("Cons1 Throughput: {}elm/{:.2?}", cnt, start.elapsed());
        });

        s.spawn(|| {
            let mut cnt = 0;
            let start = time::Instant::now();
            for _ in 0..100000 {
                match b.lock().unwrap().read() {
                    Some(_) => {
                        //println!("      R{:3}", el);
                        cnt += 1;
                    }
                    None => (), //println!("      R___"),
                }
                std::thread::sleep(time::Duration::from_nanos(1));
            }
            println!("Cons2 Throughput: {}elm/{:.2?}", cnt, start.elapsed());
        });
    });
}
