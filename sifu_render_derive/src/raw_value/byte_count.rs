

pub trait ByteCount {
    fn byte_count(&self) -> usize;
}

pub trait ByteAlign {
    fn byte_align(&self) -> usize;
}