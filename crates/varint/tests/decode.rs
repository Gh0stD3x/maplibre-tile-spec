use maplibre_tile_spec::BinaryEncoding;
use varint::VarInt;

#[test]
fn test_decode_single_byte() {
    let encoded = Vec::from([172, 2]);
    let mut decoded = Vec::new();
    
    VarInt::decode(&encoded, &mut decoded);

    assert_eq!(decoded, [300]);
}
