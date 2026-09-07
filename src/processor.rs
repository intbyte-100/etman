use crate::pipeline::PipelineSignal;

pub trait Processor {
    fn process(&self, data: PipelineSignal) -> PipelineSignal;
}