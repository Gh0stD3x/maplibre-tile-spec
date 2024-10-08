#![no_std]
#![feature(portable_simd)]
extern crate alloc;

use alloc::vec::Vec;
use maplibre_tile_spec::IntegerEncoding;

#[cfg(any(
    all(feature = "SIMDx2", any(feature = "SIMDx4", feature = "SIMDx8")),
    all(feature = "SIMDx4", any(feature = "SIMDx8", feature = "SIMDx2")),
    all(feature = "SIMDx8", any(feature = "SIMDx2", feature = "SIMDx4")),
    all(feature = "scalar", any(feature = "SIMDx2", feature = "SIMDx4", feature = "SIMDx8")),
))]
compile_error!("scalar, SIMDx2, SIMDx4 and SIMDx8 are mutually exclusive");

#[cfg(any(feature = "SIMDx2", feature = "SIMDx4", feature = "SIMDx8"))]
mod simd;
#[cfg(feature = "scalar")]
mod simple;

pub struct DeltaEncoding {}
impl IntegerEncoding<i64> for DeltaEncoding {
    fn encode(input: &[i64], output: &mut Vec<i64>) {
        assert_eq!(input.len(), output.len());
        #[cfg(any(feature = "SIMDx2", feature = "SIMDx4", feature = "SIMDx8"))]
        simd::encode_delta(input, output);

        #[cfg(feature = "scalar")]
        simple::encode_delta(input, output);
    }
    fn decode(input: &[i64], output: &mut Vec<i64>) {
        assert_eq!(input.len(), output.len());
        #[cfg(any(feature = "SIMDx2", feature = "SIMDx4", feature = "SIMDx8"))]
        simd::decode_delta(input, output);

        #[cfg(feature = "scalar")]
        simple::decode_delta(input, output);
    }
}
