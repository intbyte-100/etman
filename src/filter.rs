use crate::pipeline::PipelineSignal;

pub trait Filter {
    fn filter(&self, data: PipelineSignal) -> PipelineSignal;
}