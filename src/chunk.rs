struct RawChunk {
    raw: Vec<u8>,
}

impl RawChunk {
    fn new(size: usize) -> Self {
        Self { raw: Vec::with_capacity(size) }
    }
}

pub struct Chunk {
    data: Vec<String>
}
