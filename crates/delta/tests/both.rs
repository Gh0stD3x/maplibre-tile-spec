use delta::{encode_delta, decode_delta};

#[test]
fn test_encode_decode_delta() {
    let input = [1, 1, 1, 3, 1, 1, 2, 2, 3];
    let mut encoded = [0; 9];
    let mut decoded = [0; 9];

    encode_delta(&input, &mut encoded);
    decode_delta(&encoded, &mut decoded);

    assert_eq!(decoded, input);
}
