
use dictionary::Dictionary;
use maplibre_tile_spec::StringEncoding;

#[test]
fn test_encode_decode_dictionary() {
    let input = String::from("USA,USA,USA,USA,Mexico,Canada,Mexico,Mexico,Mexico,Argentina");

    let encoded = Dictionary::encode(&input);
    let decoded = Dictionary::decode(&encoded);

    assert_eq!(input, decoded);
}
