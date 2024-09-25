use maplibre_tile_spec::BinaryEncoding;
use varint::VarInt;

#[test]
fn test_encode_single_byte() {
    let input = [300];
    let mut encoded = Vec::new();
    let mut decoded = Vec::new();

    VarInt::encode(&input, &mut encoded);
    VarInt::decode(&encoded, &mut decoded);

    assert_eq!(decoded, input);
}
