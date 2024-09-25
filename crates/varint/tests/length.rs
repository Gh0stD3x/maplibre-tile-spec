
use varint::length_of_single_varint;

#[test]
fn test_measure_single_byte() {
    assert_eq!(length_of_single_varint(300), 2);
}
