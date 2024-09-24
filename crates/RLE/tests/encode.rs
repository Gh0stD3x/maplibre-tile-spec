use RLE::{bool::encode_bool, u8::encode_u8, u16::encode_u16, u32::encode_u32, f32::encode_f32, f64::encode_f64};

#[test]
fn test_encode_bool_rle() {
    let input = [true, true, false, false, false, true];
    let encoded = encode_bool(&input);
    assert_eq!(encoded, vec![1, 2, 0, 3, 1, 1]);
}

#[test]
fn test_encode_u8_rle() {
    let input = [1, 1, 2, 2, 2, 3];
    let encoded = encode_u8(&input);
    assert_eq!(encoded, vec![1, 2, 2, 3, 3, 1]);
}

#[test]
fn test_encode_u16_rle() {
    let input = [1, 1, 2, 2, 2, 3];
    let encoded = encode_u16(&input);
    assert_eq!(encoded, vec![1, 0, 2, 2, 0, 3, 3, 0, 1]);
}

#[test]
fn test_encode_u32_rle() {
    let input = [1, 1, 2, 2, 2, 3];
    let encoded = encode_u32(&input);
    assert_eq!(encoded, vec![1, 0, 0, 0, 2, 2, 0, 0, 0, 3, 3, 0, 0, 0, 1]);
}

#[test]
fn test_encode_f32_rle() {
    let input = [1.0, 1.0, 2.0, 2.0, 2.0, 3.0];
    let encoded = encode_f32(&input);
    assert_eq!(encoded, vec![
        0, 0, 128, 63, 2, // 1.0, count 2
        0, 0, 0, 64, 3,   // 2.0, count 3
        0, 0, 64, 64, 1   // 3.0, count 1
    ]);
}

#[test]
fn test_encode_f64_rle() {
    let input = [1.0, 1.0, 2.0, 2.0, 2.0, 3.0];
    let encoded = encode_f64(&input);
    assert_eq!(encoded, vec![
        0, 0, 0, 0, 0, 0, 240, 63, 2, // 1.0, count 2
        0, 0, 0, 0, 0, 0, 0, 64, 3,   // 2.0, count 3
        0, 0, 0, 0, 0, 0, 8, 64, 1    // 3.0, count 1
    ]);
}
