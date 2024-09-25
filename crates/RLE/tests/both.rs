use maplibre_tile_spec::BinaryEncoding;
use RLE::RunLengthEncoding;

#[test]
fn test_encode_decode_bool_rle() {
    let input = [true, true, false, false, false, true];
    let mut encoded: Vec<u8> = Vec::new();
    let mut decoded: Vec<bool> = Vec::new();

    RunLengthEncoding::encode(&input, &mut encoded);
    RunLengthEncoding::decode(&encoded, &mut decoded);
    
    assert_eq!(decoded, input);
}

#[test]
fn test_encode_decode_u8_rle() {
    let input = [1, 1, 2, 2, 2, 3];
    let mut encoded: Vec<u8> = Vec::new();
    let mut decoded: Vec<u8> = Vec::new();

    RunLengthEncoding::encode(&input, &mut encoded);
    RunLengthEncoding::decode(&encoded, &mut decoded);

    assert_eq!(decoded, input);
}

#[test]
fn test_encode_decode_u16_rle() {
    let input = [1, 1, 2, 2, 2, 3];
    let mut encoded: Vec<u8> = Vec::new();
    let mut decoded: Vec<u16> = Vec::new();

    RunLengthEncoding::encode(&input, &mut encoded);
    RunLengthEncoding::decode(&encoded, &mut decoded);

    assert_eq!(decoded, input);
}

#[test]
fn test_encode_decode_u32_rle() {
    let input = [1, 1, 2, 2, 2, 3];
    let mut encoded: Vec<u8> = Vec::new();
    let mut decoded: Vec<u32> = Vec::new();

    RunLengthEncoding::encode(&input, &mut encoded);
    RunLengthEncoding::decode(&encoded, &mut decoded);

    assert_eq!(decoded, input);
}

#[test]
fn test_encode_decode_f32_rle() {
    let input = [1.0, 1.0, 2.0, 2.0, 2.0, 3.0];
    let mut encoded: Vec<u8> = Vec::new();
    let mut decoded: Vec<f32> = Vec::new();

    RunLengthEncoding::encode(&input, &mut encoded);
    RunLengthEncoding::decode(&encoded, &mut decoded);

    assert_eq!(decoded, input);
}

#[test]
fn test_encode_decode_f64_rle() {
    let input = [1.0, 1.0, 2.0, 2.0, 2.0, 3.0];
    let mut encoded: Vec<u8> = Vec::new();
    let mut decoded: Vec<f64> = Vec::new();

    RunLengthEncoding::encode(&input, &mut encoded);
    RunLengthEncoding::decode(&encoded, &mut decoded);

    assert_eq!(decoded, input);
}
