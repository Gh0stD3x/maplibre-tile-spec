
use varint::decode_single_varint;

#[test]
fn test_decode_single_byte() {
    let encoded = Vec::from([172, 2]);
    dbg!(&encoded);
    let decoded = decode_single_varint(&encoded);

    assert_eq!(decoded, 300);
}
