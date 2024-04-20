use rodio::decoder::DecoderError;
use rodio::{PlayError, StreamError};

#[derive(thiserror::Error, Debug)]
pub enum SingerError {
    #[error("io: {0}")]
    IOError(
        #[source]
        #[from]
        std::io::Error,
    ),
    #[error("decode: {0}")]
    RodioDecoderError(
        #[source]
        #[from]
        DecoderError,
    ),
    #[error("stream: {0}")]
    RodioStreamError(
        #[source]
        #[from]
        StreamError,
    ),
    #[error("play: {0}")]
    RodioPlayError(
        #[source]
        #[from]
        PlayError,
    ),
}
