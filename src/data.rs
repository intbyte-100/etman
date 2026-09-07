use crate::stream::Stream;

pub const CHUNK_SIZE: usize = 1024 * 64;



pub struct Chunk {
    data: Vec<String>,
    size: usize
}

impl Chunk {
    pub fn new() -> Self {
        Self { data: Vec::default(), size: 0 }
    }
}
