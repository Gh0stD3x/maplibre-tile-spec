use delta::DeltaEncoding;
use maplibre_tile_spec::IntegerEncoding;

#[test]
#[cfg_attr(not(feature = "scalar"), ignore)]
fn test_decode_delta_default() {
    let input = [1, 1, 1, 3, 1, 1, 2, 2, 3];
    let mut decoded = vec![0; 9];

    DeltaEncoding::decode(&input, &mut decoded);

    assert_eq!(decoded, [1, 2, 3, 6, 7, 8, 10, 12, 15]);
}
