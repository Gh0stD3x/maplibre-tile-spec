use maplibre_tile_spec::BinaryEncoding;
use RLE::RunLengthEncoding;

#[test]
fn test_decode_bool_rle() {
    let encoded: [u8; 6] = [1, 2, 0, 3, 1, 1];
    let mut decoded: Vec<bool> = Vec::new();

    RunLengthEncoding::decode(&encoded, &mut decoded);

    assert_eq!(decoded, vec![true, true, false, false, false, true]);
}

#[test]
fn test_decode_u8_rle() {
    let encoded: [u8; 6] = [1, 2, 0, 3, 1, 1];
    let mut decoded: Vec<u8> = Vec::new();

    RunLengthEncoding::decode(&encoded, &mut decoded);

    assert_eq!(decoded, vec![1, 1, 2, 2, 2, 3]);
}

#[test]
fn test_decode_u16_rle() {
    let encoded: [u8; 9] = [1, 0, 2, 2, 0, 3, 3, 0, 1];
    let mut decoded: Vec<u16> = Vec::new();

    RunLengthEncoding::decode(&encoded, &mut decoded);

    assert_eq!(decoded, vec![1, 1, 2, 2, 2, 3]);
}

#[test]
fn test_decode_u32_rle() {
    let encoded: [u8; 15] = [1, 0, 0, 0, 2, 2, 0, 0, 0, 3, 3, 0, 0, 0, 1];
    let mut decoded: Vec<u32> = Vec::new();

    RunLengthEncoding::decode(&encoded, &mut decoded);

    assert_eq!(decoded, vec![1, 1, 2, 2, 2, 3]);
}

#[test]
fn test_decode_f32_rle() {
    let encoded: [u8; 15] = [
        0, 0, 128, 63, 2, // 1.0, count 2
        0, 0, 0, 64, 3,   // 2.0, count 3
        0, 0, 64, 64, 1   // 3.0, count 1
    ];
    let mut decoded: Vec<f32> = Vec::new();

    RunLengthEncoding::decode(&encoded, &mut decoded);

    assert_eq!(decoded, vec![1.0, 1.0, 2.0, 2.0, 2.0, 3.0]);
}

#[test]
fn test_decode_f64_rle() {
    let encoded: [u8; 27] = [
        0, 0, 0, 0, 0, 0, 240, 63, 2, // 1.0, count 2
        0, 0, 0, 0, 0, 0, 0, 64, 3,   // 2.0, count 3
        0, 0, 0, 0, 0, 0, 8, 64, 1    // 3.0, count 1
    ];
    let mut decoded: Vec<f64> = Vec::new();

    RunLengthEncoding::decode(&encoded, &mut decoded);

    assert_eq!(decoded, vec![1.0, 1.0, 2.0, 2.0, 2.0, 3.0]);
}
