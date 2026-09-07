use std::{fs::File, io::{BufRead, BufReader}};

use crate::{pipeline::{Pipeline, PipelineUnit}, stream::{ReadStream, StreamSignal}};

mod data;
mod generator;
mod filter;
mod processor;
mod stream;
mod pipeline;

fn main() {
    let stream = ReadStream::new(File::open("Cargo.toml").unwrap());

    println!("{}", size_of::<StreamSignal>());
}
