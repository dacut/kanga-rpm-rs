use crate::RPMError;
use libflate::gzip::Encoder as GzipEncoder;
use std::{
    io::{Result as IoResult, Write},
    str::FromStr,
};
use zstd::stream::Encoder as ZstdEncoder;

pub enum Compressor {
    None(Vec<u8>),
    Gzip(GzipEncoder<Vec<u8>>),
    Zstd(ZstdEncoder<'static, Vec<u8>>),
}

impl Write for Compressor {
    fn write(&mut self, content: &[u8]) -> IoResult<usize> {
        match self {
            Compressor::None(data) => data.write(content),
            Compressor::Gzip(encoder) => encoder.write(content),
            Compressor::Zstd(encoder) => encoder.write(content),
        }
    }
    fn flush(&mut self) -> IoResult<()> {
        match self {
            Compressor::None(data) => data.flush(),
            Compressor::Gzip(encoder) => encoder.flush(),
            Compressor::Zstd(encoder) => encoder.flush(),
        }
    }
}
// 19 is used here as its 19 for fedora
impl FromStr for Compressor {
    type Err = RPMError;
    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match raw {
            "none" => Ok(Compressor::None(Vec::new())),
            "gzip" => Ok(Compressor::Gzip(GzipEncoder::new(Vec::new())?)),
            "zstd" => Ok(Compressor::Zstd(ZstdEncoder::new(Vec::new(), 19)?)),
            _ => Err(RPMError::UnknownCompressorType(raw.to_string())),
        }
    }
}

impl Compressor {
    pub fn none() -> Result<Self, RPMError> {
        Ok(Compressor::None(Vec::new()))
    }

    pub fn gzip() -> Result<Self, RPMError> {
        Ok(Compressor::Gzip(GzipEncoder::new(Vec::new())?))
    }

    pub fn zstd(level: i32) -> Result<Self, RPMError> {
        Ok(Compressor::Zstd(ZstdEncoder::new(Vec::new(), level)?))
    }

    pub(crate) fn finish_compression(self) -> Result<Vec<u8>, RPMError> {
        match self {
            Compressor::None(data) => Ok(data),
            Compressor::Gzip(encoder) => Ok(encoder.finish().into_result()?),
            Compressor::Zstd(encoder) => Ok(encoder.finish().unwrap()),
        }
    }

    pub(crate) fn get_details(&self) -> Option<CompressionDetails> {
        match self {
            Compressor::None(_) => None,
            Compressor::Gzip(_) => Some(CompressionDetails {
                compression_level: "9",
                compression_name: "gzip",
            }),
            Compressor::Zstd(_) => Some(CompressionDetails {
                compression_level: "19",
                compression_name: "zstd",
            }),
        }
    }
}

pub(crate) struct CompressionDetails {
    pub(crate) compression_level: &'static str,
    pub(crate) compression_name: &'static str,
}
