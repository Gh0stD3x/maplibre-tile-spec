
// mod arch;
//
// pub use arch::*;

pub trait Decoder {
    fn decode(&mut self, in_buf: &[u32], in_pos: &mut usize, out_buf: &mut [u32], out_pos: &mut usize);
}

pub trait Encoder {
    fn encode(&mut self, in_buf: &[u32], in_pos: &mut usize, out_buf: &mut [u32], out_pos: &mut usize);
}
