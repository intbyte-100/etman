use crate::{filter::Filter, generator::Generator, processor::Processor, stream::{self, Stream}};

pub struct UnitId;

pub enum PipelineSignal {
    ProvideData(String),
    BorrowStream(Box<dyn Stream>),
    ReturnStream(Box<dyn Stream>),
    Error(Box<dyn std::error::Error>),
    End
}

pub enum PipelineUnit {
    Generator(Box<dyn Generator>),
    Processor(Box<dyn Processor>), 
    Filter(Box<dyn Filter>)
}

pub struct Pipeline {
    pipeline: Vec<PipelineUnit>
}

impl Pipeline {
    pub fn new(pipeline: Vec<PipelineUnit>) -> Self {
        Self { pipeline }
    }

    fn process(&self, stream: Box<dyn Stream>) -> PipelineSignal {
        todo!()
    }
}