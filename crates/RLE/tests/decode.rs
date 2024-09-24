use RLE::{bool::decode_bool, u8::decode_u8, u16::decode_u16, u32::decode_u32, f32::decode_f32, f64::decode_f64};

#[test]
fn test_decode_bool_rle() {
    let encoded = [1, 2, 0, 3, 1, 1];
    let decoded = decode_bool(&encoded);
    assert_eq!(decoded, vec![true, true, false, false, false, true]);
}

#[test]
fn test_decode_u8_rle() {
    let encoded = [1, 2, 2, 3, 3, 1];
    let decoded = decode_u8(&encoded);
    assert_eq!(decoded, vec![1, 1, 2, 2, 2, 3]);
}

#[test]
fn test_decode_u16_rle() {
    let encoded = [1, 0, 2, 2, 0, 3, 3, 0, 1];
    let decoded = decode_u16(&encoded);
    assert_eq!(decoded, vec![1, 1, 2, 2, 2, 3]);
}

#[test]
fn test_decode_u32_rle() {
    let encoded = [1, 0, 0, 0, 2, 2, 0, 0, 0, 3, 3, 0, 0, 0, 1];
    let decoded = decode_u32(&encoded);
    assert_eq!(decoded, vec![1, 1, 2, 2, 2, 3]);
}

#[test]
fn test_decode_f32_rle() {
    let encoded = [
        0, 0, 128, 63, 2, // 1.0, count 2
        0, 0, 0, 64, 3,   // 2.0, count 3
        0, 0, 64, 64, 1   // 3.0, count 1
    ];
    let decoded = decode_f32(&encoded);
    assert_eq!(decoded, vec![1.0, 1.0, 2.0, 2.0, 2.0, 3.0]);
}

#[test]
fn test_decode_f64_rle() {
    let encoded = [
        0, 0, 0, 0, 0, 0, 240, 63, 2, // 1.0, count 2
        0, 0, 0, 0, 0, 0, 0, 64, 3,   // 2.0, count 3
        0, 0, 0, 0, 0, 0, 8, 64, 1    // 3.0, count 1
    ];
    let decoded = decode_f64(&encoded);
    assert_eq!(decoded, vec![1.0, 1.0, 2.0, 2.0, 2.0, 3.0]);
}
