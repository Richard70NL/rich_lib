//! This module includes an implementation of a buffered stream with both a buffered reader
//! and writer.

/************************************************************************************************/
/************************************************************************************************/
/************************************************************************************************/

use std::io::BufReader;
use std::io::BufWriter;
use std::net::TcpStream;
use std::net::ToSocketAddrs;

/************************************************************************************************/
/************************************************************************************************/
/************************************************************************************************/

/// A buffered stream which includes both a buffered reader and writer at the same time.
pub struct BufStream<'a> {
    stream: TcpStream,
    reader: BufReader<&'a TcpStream>,
    writer: BufWriter<&'a TcpStream>,
}

/************************************************************************************************/
/************************************************************************************************/
/************************************************************************************************/

impl<'a> BufStream<'a> {
    /*------------------------------------------------------------------------------------------*/

    pub fn new<A: ToSocketAddrs>(addr: A) -> Result<Self, std::io::Error> {
        Ok(Self::from(TcpStream::connect(addr)?))
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn from(stream: TcpStream) -> Self {
        BufStream {
            stream: stream,
            reader: BufReader::new(&stream),
            writer: BufWriter::new(&stream),
        }
    }

    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/
/************************************************************************************************/
/************************************************************************************************/