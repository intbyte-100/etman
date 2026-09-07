use std::{
    fmt::{Display, write},
    fs::File,
    io::{BufRead, BufReader, ErrorKind, Read, Seek},
    result,
};

use crate::data::CHUNK_SIZE;

pub type Searcher = dyn Fn(&[u8]) -> Option<usize>;

#[derive(Debug)]
pub struct ChainIndex(usize);

#[derive(Debug)]
pub enum Error {
    StreamEnded,
    NotMaterializable,
    Other(Box<dyn std::error::Error>),
}

#[derive(Debug)]
pub enum StreamSignal {
    MaterializedData(String),
    Unchained(String, ChainIndex),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::StreamEnded => write!(f, "StreamEnded"),
            Error::Other(error) => write!(f, "{}", error),
            Error::NotMaterializable => write!(f, "StreamEnded"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::StreamEnded | Error::NotMaterializable => None,
            Error::Other(error) => Some(error.as_ref()),
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

pub trait Stream {
    fn read_until(&mut self, searcher: &Searcher) -> Result<StreamSignal, Error>;
    fn try_materialize(&mut self, searcher: &Searcher) -> Result<String, Error>;
    fn chained_searchers(self: Box<Self>) -> Box<dyn ChainedSearchStream>;
}

pub trait ChainedSearchStream: Stream {
    fn chain(self: Box<Self>, searcher: Box<Searcher>) -> Box<dyn ChainedSearchStream>;
}

struct ChainedSeacrhStreamStruct<T: BufRead> {
    read: T,
    searchers: Vec<Box<Searcher>>,
}

impl<T: BufRead> Stream for ChainedSeacrhStreamStruct<T> {
    fn read_until(&mut self, searcher: &Searcher) -> Result<StreamSignal, Error> {
        todo!()
    }

    fn try_materialize(&mut self, searcher: &Searcher) -> Result<String, Error> {
        todo!()
    }

    fn chained_searchers(self: Box<Self>) -> Box<dyn ChainedSearchStream> {
        todo!()
    }
}

impl<T: BufRead + 'static> ChainedSearchStream for ChainedSeacrhStreamStruct<T> {
    fn chain(mut self: Box<Self>, searcher: Box<Searcher>) -> Box<dyn ChainedSearchStream> {
        self.searchers.push(searcher);
        self
    }
}

fn read_until<F>(
    buf_reader: &mut BufReader<impl Read>,
    limit: usize,
    mut handler: F,
) -> Result<StreamSignal, Error>
where
    F: FnMut(&[u8], &mut Vec<u8>) -> (bool, usize, Option<ChainIndex>),
{
    let mut buf = Vec::new();
    let mut read = 0;

    loop {
        let (done, used, chain_index) = {
            let available = match buf_reader.fill_buf() {
                Ok(n) => n,
                Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => return Err(Error::Other(Box::new(e))),
            };

            handler(available, &mut buf)
        };

        if read > limit {
            todo!()
        }

        buf_reader.consume(used);
        read += used;

        if done || used == 0 {
            let string = String::from_utf8(buf).map_err(|it| Error::Other(Box::from(it)))?;

            let signal = if let Some(index) = chain_index {
                StreamSignal::Unchained(string, index)
            } else {
                StreamSignal::MaterializedData(string)
            };

            return Ok(signal);
        }
    }
}


pub struct ReadStream {
    buf_reader: BufReader<Box<dyn Read>>,
}

impl ReadStream {
    pub fn new(file: File) -> Self {
        Self {
            buf_reader: BufReader::new(Box::new(file)),
        }
    }

    fn inner_read_until(&mut self, limit: usize, searcher: &Searcher) -> Result<StreamSignal, Error> {
        read_until(
            &mut self.buf_reader,
            limit,
            |available, buf| match searcher(available) {
                Some(i) => {
                    buf.extend_from_slice(&available[..=i]);
                    (true, i + 1, Some(ChainIndex(i)))
                }
                None => {
                    buf.extend_from_slice(available);
                    (false, available.len(), None)
                }
            },
        )
    }
}

impl Stream for ReadStream {
    fn read_until(&mut self, searcher: &Searcher) -> Result<StreamSignal, Error> {
        self.inner_read_until(CHUNK_SIZE, searcher)
    }

    fn try_materialize(&mut self, searcher: &Searcher) -> Result<String, Error> {
        if let StreamSignal::MaterializedData(value) =
            self.inner_read_until(CHUNK_SIZE * 4, searcher)?
        {
            Ok(value)
        } else {
            Err(Error::NotMaterializable)
        }
    }

    fn chained_searchers(self: Box<Self>) -> Box<dyn ChainedSearchStream> {
        todo!()
    }
}


pub struct PrefixedChaindedStream {
    
} 

impl PrefixedChaindedStream {
    pub fn new(prefix: String, stream: Box<dyn Stream>, len: usize) -> Self {
        Self {  }
    }
}
impl Stream for PrefixedChaindedStream {
    fn read_until(&mut self, searcher: &Searcher) -> Result<StreamSignal, Error> {
        todo!()
    }

    fn try_materialize(&mut self, searcher: &Searcher) -> Result<String, Error> {
        todo!()
    }

    fn chained_searchers(self: Box<Self>) -> Box<dyn ChainedSearchStream> {
        todo!()
    }
}