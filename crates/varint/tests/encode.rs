use maplibre_tile_spec::BinaryEncoding;
use varint::VarInt;

#[test]
fn test_encode_single_byte() {
    let input = [300];
    let mut encoded = Vec::new();
    
    VarInt::encode(&input, &mut encoded);

    assert_eq!(encoded, vec![0xAC, 0x02]);
}
