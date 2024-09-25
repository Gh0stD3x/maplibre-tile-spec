#![no_std]
#![allow(non_snake_case)]

extern crate alloc;

use alloc::vec::Vec;
use maplibre_tile_spec::BinaryEncoding;

mod bool;
mod u8;
mod u16;
mod u32;
mod f32;
mod f64;


pub struct RunLengthEncoding {}

impl BinaryEncoding<bool> for RunLengthEncoding {
    fn encode(input: &[bool], output: &mut Vec<u8>) {
        let mut encoded = bool::encode_bool(input);
        output.append(&mut encoded);
    }
    fn decode(input: &[u8], output: &mut Vec<bool>) {
        let mut encoded = bool::decode_bool(input);
        output.append(&mut encoded);
    }
}

impl BinaryEncoding<u8> for RunLengthEncoding {
    fn encode(input: &[u8], output: &mut Vec<u8>) {
        let mut encoded = u8::encode_u8(input);
        output.append(&mut encoded);
    }
    fn decode(input: &[u8], output: &mut Vec<u8>) {
        let mut encoded = u8::decode_u8(input);
        output.append(&mut encoded);
    }
}
impl BinaryEncoding<u16> for RunLengthEncoding {
    fn encode(input: &[u16], output: &mut Vec<u8>) {
        let mut encoded = u16::encode_u16(input);
        output.append(&mut encoded);
    }
    fn decode(input: &[u8], output: &mut Vec<u16>) {
        let mut encoded = u16::decode_u16(input);
        output.append(&mut encoded);
    }
}
impl BinaryEncoding<u32> for RunLengthEncoding {
    fn encode(input: &[u32], output: &mut Vec<u8>) {
        let mut encoded = u32::encode_u32(input);
        output.append(&mut encoded);
    }
    fn decode(input: &[u8], output: &mut Vec<u32>) {
        let mut encoded = u32::decode_u32(input);
        output.append(&mut encoded);
    }
}

impl BinaryEncoding<f32> for RunLengthEncoding {
    fn encode(input: &[f32], output: &mut Vec<u8>) {
        let mut encoded = f32::encode_f32(input);
        output.append(&mut encoded);
    }
    fn decode(input: &[u8], output: &mut Vec<f32>) {
        let mut encoded = f32::decode_f32(input);
        output.append(&mut encoded);
    }
}
impl BinaryEncoding<f64> for RunLengthEncoding {
    fn encode(input: &[f64], output: &mut Vec<u8>) {
        let mut encoded = f64::encode_f64(input);
        output.append(&mut encoded);
    }
    fn decode(input: &[u8], output: &mut Vec<f64>) {
        let mut encoded = f64::decode_f64(input);
        output.append(&mut encoded);
    }
}
