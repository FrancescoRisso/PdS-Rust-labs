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
