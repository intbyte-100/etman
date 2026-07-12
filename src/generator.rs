use crate::data::Data;

trait Generator {
    fn next(&mut self) -> Data;
}