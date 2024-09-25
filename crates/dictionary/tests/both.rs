
use dictionary::{encode_dictionary, decode_dictionary};

#[test]
fn test_encode_decode_dictionary() {
    let input = String::from("USA,USA,USA,USA,Mexico,Canada,Mexico,Mexico,Mexico,Argentina");

    let encoded = encode_dictionary(&input);
    let decoded = decode_dictionary(&encoded);

    assert_eq!(input, decoded);
}
