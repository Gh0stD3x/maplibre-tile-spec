use RLE::{
    bool::{encode_bool, decode_bool},
    u8::{encode_u8, decode_u8},
    u16::{encode_u16, decode_u16},
    u32::{encode_u32, decode_u32},
    f32::{encode_f32, decode_f32},
    f64::{encode_f64, decode_f64}
};

#[test]
fn test_encode_decode_bool_rle() {
    let input = [true, true, false, false, false, true];
    let encoded = encode_bool(&input);
    let decoded = decode_bool(&encoded);
    assert_eq!(decoded, input);
}

#[test]
fn test_encode_decode_u8_rle() {
    let input = [1, 1, 2, 2, 2, 3];
    let encoded = encode_u8(&input);
    let decoded = decode_u8(&encoded);
    assert_eq!(decoded, input);
}

#[test]
fn test_encode_decode_u16_rle() {
    let input = [1, 1, 2, 2, 2, 3];
    let encoded = encode_u16(&input);
    let decoded = decode_u16(&encoded);
    assert_eq!(decoded, input);
}

#[test]
fn test_encode_decode_u32_rle() {
    let input = [1, 1, 2, 2, 2, 3];
    let encoded = encode_u32(&input);
    let decoded = decode_u32(&encoded);
    assert_eq!(decoded, input);
}

#[test]
fn test_encode_decode_f32_rle() {
    let input = [1.0, 1.0, 2.0, 2.0, 2.0, 3.0];
    let encoded = encode_f32(&input);
    let decoded = decode_f32(&encoded);
    assert_eq!(decoded, input);
}

#[test]
fn test_encode_decode_f64_rle() {
    let input = [1.0, 1.0, 2.0, 2.0, 2.0, 3.0];
    let encoded = encode_f64(&input);
    let decoded = decode_f64(&encoded);
    assert_eq!(decoded, input);
}
