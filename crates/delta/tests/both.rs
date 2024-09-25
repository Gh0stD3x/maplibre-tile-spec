use delta::DeltaEncoding;
use maplibre_tile_spec::IntegerEncoding;

#[test]
fn test_encode_decode_delta() {
    let input = [1, 1, 1, 3, 1, 1, 2, 2, 3];
    let mut encoded = vec![0; 9];
    let mut decoded = vec![0; 9];

    DeltaEncoding::encode(&input, &mut encoded);
    DeltaEncoding::decode(&encoded, &mut decoded);

    assert_eq!(decoded, input);
}
