use crate::data::Data;

trait Processor {
    fn process(&self, data: Data) -> Data;
}