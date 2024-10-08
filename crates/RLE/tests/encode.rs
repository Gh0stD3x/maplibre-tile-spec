use maplibre_tile_spec::BinaryEncoding;
use RLE::RunLengthEncoding;

#[test]
fn test_encode_bool_rle() {
    let input: [bool; 6] = [true, true, false, false, false, true];
    let mut encoded = Vec::new();

    RunLengthEncoding::encode(&input, &mut encoded);
    
    assert_eq!(encoded, vec![1, 2, 0, 3, 1, 1]);
}

#[test]
fn test_encode_u8_rle() {
    let input: [u32; 6] = [1, 1, 2, 2, 2, 3];
    let mut encoded = Vec::new();
    
    RunLengthEncoding::encode(&input, &mut encoded);
    
    assert_eq!(encoded, vec![1, 2, 2, 3, 3, 1]);
}

#[test]
fn test_encode_u16_rle() {
    let input: [u16; 6] = [1, 1, 2, 2, 2, 3];
    let mut encoded = Vec::new();

    RunLengthEncoding::encode(&input, &mut encoded);

    assert_eq!(encoded, vec![1, 0, 2, 2, 0, 3, 3, 0, 1]);
}

#[test]
fn test_encode_u32_rle() {
    let input: [u32; 6] = [1, 1, 2, 2, 2, 3];
    let mut encoded = Vec::new();

    RunLengthEncoding::encode(&input, &mut encoded);

    assert_eq!(encoded, vec![1, 0, 0, 0, 2, 2, 0, 0, 0, 3, 3, 0, 0, 0, 1]);
}

#[test]
fn test_encode_f32_rle() {
    let input: [f32; 6] = [1.0, 1.0, 2.0, 2.0, 2.0, 3.0];
    let mut encoded = Vec::new();

    RunLengthEncoding::encode(&input, &mut encoded);

    assert_eq!(encoded, vec![
        0, 0, 128, 63, 2, // 1.0, count 2
        0, 0, 0, 64, 3,   // 2.0, count 3
        0, 0, 64, 64, 1   // 3.0, count 1
    ]);
}

#[test]
fn test_encode_f64_rle() {
    let input: [f64; 6] = [1.0, 1.0, 2.0, 2.0, 2.0, 3.0];
    let mut encoded = Vec::new();

    RunLengthEncoding::encode(&input, &mut encoded);

    assert_eq!(encoded, vec![
        0, 0, 0, 0, 0, 0, 240, 63, 2, // 1.0, count 2
        0, 0, 0, 0, 0, 0, 0, 64, 3,   // 2.0, count 3
        0, 0, 0, 0, 0, 0, 8, 64, 1    // 3.0, count 1
    ]);
}
