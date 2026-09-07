use std::string;

use crate::{pipeline::PipelineSignal, stream::{ChainedSearchStream, Stream, StreamSignal}};

pub trait Generator {
    fn next(&mut self, source: Box<dyn Stream>) -> PipelineSignal;
}


pub struct LineGenerator;

impl Generator for LineGenerator {
    fn next(&mut self, mut source: Box<dyn Stream>) -> PipelineSignal {
        fn searcher(buf: &[u8]) -> Option<usize> {
            memchr::memchr(b'\n', buf)
        }
        
        let signal = source.read_until(&searcher);



        match signal {
            Ok(signal) => match signal {
                StreamSignal::MaterializedData(string) => PipelineSignal::ProvideData(string),
                StreamSignal::Unchained(prefix, chain_index) => todo!(),
            },
            Err(_) => todo!(),
        }
    }
}