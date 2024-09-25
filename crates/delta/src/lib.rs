#![no_std]
#![feature(portable_simd)]

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

/// # Info
/// Encodes the input slice with delta encoding in a performant manor with SIMD support
pub fn encode_delta(input: &[i32], output: &mut [i32]) {
    assert_eq!(input.len(), output.len());
    #[cfg(any(feature = "SIMDx2", feature = "SIMDx4", feature = "SIMDx8"))]
    simd::encode_delta(input, output);

    #[cfg(feature = "scalar")]
    simple::encode_delta(input, output);
}

/// # Info
/// Decodes the input slice with delta encoding in a performant manor with SIMD support
pub fn decode_delta(input: &[i32], output: &mut [i32]) {
    assert_eq!(input.len(), output.len());
    #[cfg(any(feature = "SIMDx2", feature = "SIMDx4", feature = "SIMDx8"))]
    simd::decode_delta(input, output);

    #[cfg(feature = "scalar")]
    simple::decode_delta(input, output);
}
