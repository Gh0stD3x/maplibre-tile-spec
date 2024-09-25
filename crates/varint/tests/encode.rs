
use varint::encode_single_varint;

#[test]
fn test_encode_single_byte() {
    let encoded = encode_single_varint(300);

    assert_eq!(encoded.0, vec![0xAC, 0x02]);
    assert_eq!(encoded.1, 2);
}
