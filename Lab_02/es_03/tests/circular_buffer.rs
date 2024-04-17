use es_03::CircularBuffer;

#[test]
fn write_until_error() {
    let mut buf: CircularBuffer<u32> = CircularBuffer::new(2);

    let ins1 = buf.write(1);
    let ins2 = buf.write(2);
    let ins3 = buf.write(3);

    assert!(ins1.is_ok());
    assert!(ins2.is_ok());
    assert!(ins3.is_err());
}

#[test]
fn read_until_error() {
    let mut buf: CircularBuffer<u32> = CircularBuffer::new(2);

    let _ = buf.write(1);
    let _ = buf.write(2);

    assert_eq!(buf.read(), Some(1));
    assert_eq!(buf.read(), Some(2));
    assert_eq!(buf.read(), None);
}

#[test]
fn write_and_clear() {
    let mut buf: CircularBuffer<u32> = CircularBuffer::new(2);

    let _ = buf.write(1);
    let _ = buf.write(2);

    buf.clear();

    let ins1 = buf.write(1);
    let ins2 = buf.write(2);
    let ins3 = buf.write(3);

    assert!(ins1.is_ok());
    assert!(ins2.is_ok());
    assert!(ins3.is_err());
}

#[test]
fn read_with_clear() {
    let mut buf: CircularBuffer<u32> = CircularBuffer::new(2);

    let _ = buf.write(1);

    buf.clear();

    let _ = buf.write(2);

    assert_eq!(buf.read(), Some(2));
    assert_eq!(buf.read(), None);
}

#[test]
fn get_size() {
    let buf: CircularBuffer<u32> = CircularBuffer::new(2);
    assert_eq!(buf.size(), 2)
}

#[test]
fn overwrite() {
    let mut buf: CircularBuffer<u32> = CircularBuffer::new(2);

    let _ = buf.write(1);
    let _ = buf.write(2);
    let _ = buf.overwrite(3);

    assert_eq!(buf.read(), Some(2));
    assert_eq!(buf.read(), Some(3));
}

#[test]
fn make_contiguos() {
    let mut buf: CircularBuffer<u32> = CircularBuffer::new(3);

    let _ = buf.write(1);
    let _ = buf.write(2);
    let _ = buf.write(3);

    assert_eq!(buf.get_head(), 0);

    assert_eq!(buf.read(), Some(1));
    assert_eq!(buf.get_head(), 1);
    let _ = buf.write(4);

    assert_eq!(buf.read(), Some(2));
    assert_eq!(buf.get_head(), 2);

    buf.make_contiguos();

    assert_eq!(buf.get_head(), 0);
    assert_eq!(buf.read(), Some(3));
    assert_eq!(buf.get_head(), 1);
    assert_eq!(buf.read(), Some(4));
    assert_eq!(buf.get_head(), 2);
}

#[test]
fn index() {
    let mut buf: CircularBuffer<u32> = CircularBuffer::new(3);

    let _ = buf.write(1);
    let _ = buf.write(2);
    let _ = buf.write(3);
    buf.read();

    let ptr1 = buf[0];
    let ptr2 = buf[1];

    let pan = std::panic::catch_unwind(|| {
        buf[2];
    });

    assert_eq!(ptr1, 2);
    assert_eq!(ptr2, 3);
    assert!(pan.is_err());
}
