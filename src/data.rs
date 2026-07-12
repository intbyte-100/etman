use crate::chunk::Chunk;

pub trait StreamData {
    
}

pub enum Data {
    Finite(Chunk),
    Stream(Box<dyn StreamData>)
}