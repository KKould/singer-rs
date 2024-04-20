use std::io::Error;
use rodio::decoder::DecoderError;
use rodio::{PlayError, StreamError};

#[derive(thiserror::Error, Debug)]
pub enum SingerError {
    #[error("io: {0}")]
    IO(
        #[source]
        #[from]
        Error,
    ),
    #[error("decode: {0}")]
    Decoder(
        #[source]
        #[from]
        DecoderError,
    ),
    #[error("stream: {0}")]
    Stream(
        #[source]
        #[from]
        StreamError,
    ),
    #[error("play: {0}")]
    Play(
        #[source]
        #[from]
        PlayError,
    ),
}
