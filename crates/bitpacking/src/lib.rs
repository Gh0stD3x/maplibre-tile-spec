#![no_std]
extern crate alloc;

mod fastpack;
mod fastunpack;
mod fastpackwithoutmask;

#[allow(non_snake_case)]
pub mod BitPacking {
    use super::fastpack::fastpack as _fastpack;
    use super::fastunpack::fastunpack as _fastunpack;
    use super::fastpackwithoutmask::fastpackwithoutmask as _fastpackwithoutmask;

    pub fn fastpack(input: &[u32; 32], output: &mut [u32; 32], bit: u8) {
        _fastpack(input, 0, output, 0, bit);
    }
    pub fn fastunpack(input: &[u32; 32], output: &mut [u32; 32], bit: u8) {
        _fastunpack(input, 0, output, 0, bit);
    }
    pub fn fastpackwithoutmask(input: &[u32; 32], output: &mut [u32; 32], bit: u8) {
        _fastpackwithoutmask(input, 0, output, 0, bit);
    }
}
