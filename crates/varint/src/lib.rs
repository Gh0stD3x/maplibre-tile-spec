#![no_std]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

use maplibre_tile_spec::BinaryEncoding;


const N1: i64 = 2_i64.pow( 7);
const N2: i64 = 2_i64.pow(14);
const N3: i64 = 2_i64.pow(21);
const N4: i64 = 2_i64.pow(28);
const N5: i64 = 2_i64.pow(35);
const N6: i64 = 2_i64.pow(42);
const N7: i64 = 2_i64.pow(49);
const N8: i64 = 2_i64.pow(56);
const N9: i64 = 2_i64.wrapping_pow(63);


const MSB: i64 = 0x80;
const REST: i64 = 0x7F;
const MSBALL: i64 = !REST;
const INT: i64 = 2_i64.pow(31);



pub struct VarInt {}
impl VarInt {
    pub fn length(number: i64) -> u8 {
        match number {
            n if n < N1 => 1,
            n if n < N2 => 2,
            n if n < N3 => 3,
            n if n < N4 => 4,
            n if n < N5 => 5,
            n if n < N6 => 6,
            n if n < N7 => 7,
            n if n < N8 => 8,
            n if n < N9 => 9,
            _ => 10,
        }
    }
    pub fn encode_single_varint(number: i64) -> Vec<u8> {
        let mut output = vec![0; Self::length(number) as usize];
        let mut number = number;
        let mut offset = 0;

        while number >= INT {
            output[offset] = ((number & 0xFF) | MSB) as u8;
            offset += 1;
            number /= 128;
        }
        while number & MSBALL != 0 {
            output[offset] = ((number & 0xFF) | MSB) as u8;
            offset += 1;
            number >>= 7;
        }
        output[offset] = (number | 0) as u8;

        output
    }
    pub fn decode_single_varint(buffer: &[u8]) -> i64 {
        let mut res = 0;
        let mut shift = 0;
        let mut counter = 0;
        let l = buffer.len();

        // This is a Do-While
        while {
            assert!(counter <= l);
            assert!(shift < 49);

            let b = buffer[counter] as i64;
            counter += 1;
            res += if shift < 28 { (b & REST) << shift } else { (b & REST) * 2_i64.pow(shift) };
            shift += 7;

            b >= MSB
        } {}

        res
    }
}
impl BinaryEncoding<i64> for VarInt {
    fn encode(input: &[i64], output: &mut Vec<u8>) {
        for value in input {
            let data = Self::encode_single_varint(*value);

            for d in &data { output.push(*d); }
        }
    }
    fn decode(input: &[u8], output: &mut Vec<i64>) {
        let mut offset = 0;
        while offset < input.len() {
            let number = Self::decode_single_varint(&input[offset..]);

            output.push(number);

            offset += Self::length(number) as usize;
        }
    }
}
