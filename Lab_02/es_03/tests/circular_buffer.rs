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
