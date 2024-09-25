
// mod arch;
//
// pub use arch::*;

pub trait BinaryEncoding<T> {
    fn encode(input: &[T], output: &mut Vec<u8>);
    fn decode(input: &[u8], output: &mut Vec<T>);
}
pub trait StringEncoding {
    fn encode(input: &String) -> String;
    fn decode(input: &String) -> String;
}
pub trait IntegerEncoding<T> {
    fn encode(input: &[T], output: &mut Vec<T>);
    fn decode(input: &[T], output: &mut Vec<T>);
}
