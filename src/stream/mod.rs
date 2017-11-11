//! Provides an abstraction over the standard `TcpStream` and `native_tls`'s `TlsStream`.

pub mod tcp;
pub mod tls;
pub mod utils;

pub use self::tcp::TcpStream;
pub use self::tls::TlsStream;
pub use std::io::Error as IoError;
use mio::Poll;
use mio::Token;
use native_tls;
use self::tcp::TcpError;
use self::tls::TlsError;
use std::io::Write;
use std::io;
use std::result;

pub enum Stream<'poll> {
    Tcp(TcpStream<'poll>),
    Tls(TlsStream<'poll>),
}

#[derive(Debug)]
pub enum StreamErr {
    IoError(IoError),
    CantResolveAddr,
    TlsError(native_tls::Error),
    ConnectionClosed,
}

pub type Result<T> = result::Result<T, StreamErr>;

impl From<TcpError> for StreamErr {
    fn from(tcp_err: TcpError) -> StreamErr {
        match tcp_err {
            TcpError::IoError(io_err) =>
                StreamErr::IoError(io_err),
            TcpError::CantResolveAddr =>
                StreamErr::CantResolveAddr,
        }
    }
}

impl From<TlsError> for StreamErr {
    fn from(tls_err: TlsError) -> StreamErr {
        match tls_err {
            TlsError::TcpError(err) =>
                StreamErr::from(err),
            TlsError::TlsError(err) =>
                StreamErr::TlsError(err),
            TlsError::ConnectionClosed =>
                StreamErr::ConnectionClosed,
        }
    }
}

impl From<IoError> for StreamErr {
    fn from(io_err: IoError) -> StreamErr {
        StreamErr::IoError(io_err)
    }
}

impl<'poll> Stream<'poll> {
    pub fn new_tcp(poll: &'poll Poll, serv_addr: &str, serv_port: u16) -> Result<Stream<'poll>> {
        TcpStream::new(poll, serv_addr, serv_port)
            .map_err(StreamErr::from)
            .map(Stream::Tcp)
    }

    pub fn new_tls(
        poll: &'poll Poll,
        serv_addr: &str,
        serv_port: u16,
        domain: &str,
    ) -> Result<Stream<'poll>> {
        TlsStream::new(poll, serv_addr, serv_port, domain)
            .map_err(StreamErr::from)
            .map(Stream::Tls)
    }

    pub fn write_ready(&mut self) -> Result<()> {
        match *self {
            Stream::Tcp(ref mut s) =>
                s.write_ready().map_err(StreamErr::from),
            Stream::Tls(ref mut s) =>
                s.write_ready().map_err(StreamErr::from),
        }
    }

    pub fn read_ready(&mut self, buf: &mut [u8]) -> Result<usize> {
        match *self {
            Stream::Tcp(ref mut s) =>
                s.read_ready(buf).map_err(StreamErr::from),
            Stream::Tls(ref mut s) =>
                s.read_ready(buf).map_err(StreamErr::from),
        }
    }

    pub fn get_tok(&self) -> Token {
        match *self {
            Stream::Tcp(ref s) =>
                s.get_tok(),
            Stream::Tls(ref s) =>
                s.get_tok(),
        }
    }
}

impl<'poll> Write for Stream<'poll> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match *self {
            Stream::Tcp(ref mut s) =>
                s.write(buf),
            Stream::Tls(ref mut s) =>
                s.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match *self {
            Stream::Tcp(ref mut s) =>
                s.flush(),
            Stream::Tls(ref mut s) =>
                s.flush(),
        }
    }
}
