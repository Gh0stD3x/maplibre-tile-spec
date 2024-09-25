
use varint::{encode_single_varint, decode_single_varint};

#[test]
fn test_encode_single_byte() {
    let input = 300;

    let encoded = encode_single_varint(input);
    let decoded = decode_single_varint(&encoded.0);

    assert_eq!(decoded, 300);
}
